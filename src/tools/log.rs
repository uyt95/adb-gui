//! A record of the commands the app runs, shown in the console panel.
//!
//! Worker threads add and finish records; the main thread reads snapshots and
//! is told about changes through [`CommandLog::subscribe`].

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use gtk::glib;

/// How many commands the log keeps. Older ones are dropped first.
pub const MAX_RECORDS: usize = 100;
/// How much of stdout and of stderr is kept per command. The rest is dropped,
/// so one command with a huge output cannot fill the memory.
pub const MAX_STREAM_BYTES: usize = 64 * 1024;

/// One command the app ran or is running.
#[derive(Clone, Debug)]
pub struct Record {
    pub id: u64,
    /// When the command was sent, in local time.
    pub sent: glib::DateTime,
    pub program: String,
    pub arguments: Vec<String>,
    pub state: RecordState,
    /// Changes whenever the record changes, so the console can skip rows that did not.
    pub revision: u64,
}

#[derive(Clone, Debug)]
pub enum RecordState {
    Running,
    /// Started in the background without waiting for it; its output is not captured.
    Launched,
    /// The program could not be started at all.
    NotStarted(String),
    Finished(Finished),
}

#[derive(Clone, Debug)]
pub struct Finished {
    pub duration: Duration,
    /// `None` when the process was stopped by a signal.
    pub exit_code: Option<i32>,
    pub stdout: Stream,
    pub stderr: Stream,
    /// The command exited with 0, but the app did not accept its output.
    pub rejected: bool,
}

/// Captured output, cut to [`MAX_STREAM_BYTES`].
#[derive(Clone, Debug, Default)]
pub struct Stream {
    pub text: String,
    /// Bytes dropped from the end.
    pub dropped: usize,
}

impl Stream {
    fn new(text: &str) -> Stream {
        if text.len() <= MAX_STREAM_BYTES {
            return Stream {
                text: text.to_string(),
                dropped: 0,
            };
        }
        let mut end = MAX_STREAM_BYTES;
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        Stream {
            text: text[..end].to_string(),
            dropped: text.len() - end,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty() && self.dropped == 0
    }
}

impl Record {
    pub fn is_failed(&self) -> bool {
        match &self.state {
            RecordState::Running | RecordState::Launched => false,
            RecordState::NotStarted(_) => true,
            RecordState::Finished(finished) => finished.rejected || finished.exit_code != Some(0),
        }
    }

    /// The command as one line, with the program's file name instead of its full path.
    pub fn short_command(&self) -> String {
        let name = Path::new(&self.program)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| self.program.clone());
        command_line(&name, &self.arguments)
    }

    /// The command as one line, with the program's full path.
    pub fn full_command(&self) -> String {
        command_line(&self.program, &self.arguments)
    }

    /// The send time with milliseconds, e.g. `2026-11-23 15:22:31.118`.
    pub fn sent_text(&self) -> String {
        let date = self
            .sent
            .format("%Y-%m-%d %H:%M:%S")
            .map(|text| text.to_string())
            .unwrap_or_default();
        format!("{date}.{:03}", self.sent.microsecond() / 1000)
    }

    /// The record as plain text, for copying.
    pub fn to_text(&self) -> String {
        let mut text = format!("[{}] $ {}\n", self.sent_text(), self.full_command());
        match &self.state {
            RecordState::Running => text.push_str("running\n"),
            RecordState::Launched => text.push_str("launched in the background\n"),
            RecordState::NotStarted(error) => text.push_str(&format!("not started: {error}\n")),
            RecordState::Finished(finished) => {
                let exit = finished
                    .exit_code
                    .map_or("stopped by a signal".to_string(), |code| {
                        format!("exit {code}")
                    });
                text.push_str(&format!("{exit}, {}", format_duration(finished.duration)));
                if finished.rejected {
                    text.push_str(", output not accepted");
                }
                text.push('\n');
                for (name, stream) in [("stdout", &finished.stdout), ("stderr", &finished.stderr)] {
                    if stream.is_empty() {
                        continue;
                    }
                    text.push_str(&format!("{name}:\n{}", stream.text));
                    if !stream.text.ends_with('\n') {
                        text.push('\n');
                    }
                    if stream.dropped > 0 {
                        text.push_str(&format!("({} more bytes not kept)\n", stream.dropped));
                    }
                }
            }
        }
        text
    }
}

/// `182 ms` below a second, `5.0 s` from there.
pub fn format_duration(duration: Duration) -> String {
    let ms = duration.as_millis();
    if ms < 1000 {
        format!("{ms} ms")
    } else {
        format!("{:.1} s", duration.as_secs_f64())
    }
}

/// Joins the words of a command for display, quoting words that a shell would split.
fn command_line(program: &str, arguments: &[String]) -> String {
    std::iter::once(program)
        .chain(arguments.iter().map(String::as_str))
        .map(display_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn display_word(word: &str) -> String {
    // Text for `adb shell` is already quoted for the device shell; show it as
    // is. Only a word that is exactly what `shell_quote` makes counts, so
    // something like `'a' 'b'` is still quoted as one word.
    let already_quoted = word.len() >= 2
        && word.starts_with('\'')
        && word.ends_with('\'')
        && super::shell_quote(&word[1..word.len() - 1].replace(r"'\''", "'")) == word;
    let needs_quotes = word.is_empty()
        || word
            .chars()
            .any(|c| c.is_whitespace() || "\"'\\$`;&|<>()*?!#~[]{}".contains(c));
    if needs_quotes && !already_quoted {
        super::shell_quote(word)
    } else {
        word.to_string()
    }
}

#[derive(Default)]
struct Data {
    records: VecDeque<Record>,
    next_id: u64,
    next_revision: u64,
    /// Commands that failed since the app started, also those cleared or
    /// dropped since. Lets the window tell whether a new failure came in.
    failed_total: u64,
}

impl Data {
    fn revision(&mut self) -> u64 {
        self.next_revision += 1;
        self.next_revision
    }
}

#[derive(Default)]
struct Inner {
    data: Mutex<Data>,
    /// A change notification is queued on the main loop and not yet delivered.
    wake_pending: AtomicBool,
}

thread_local! {
    /// Change listeners; only the main thread registers any.
    static LISTENERS: RefCell<Vec<Box<dyn Fn()>>> = RefCell::default();
}

/// The shared command log. Cloning it gives another handle to the same log.
#[derive(Clone, Default)]
pub struct CommandLog {
    inner: Arc<Inner>,
}

impl fmt::Debug for CommandLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CommandLog")
    }
}

impl CommandLog {
    fn data(&self) -> MutexGuard<'_, Data> {
        // A panic while holding the lock leaves consistent data behind.
        self.inner
            .data
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Records that a command is being sent. Finish it through the returned entry.
    pub fn start(&self, program: &str, arguments: &[String]) -> Entry {
        let id = {
            let mut data = self.data();
            data.next_id += 1;
            let id = data.next_id;
            let revision = data.revision();
            data.records.push_back(Record {
                id,
                sent: glib::DateTime::now_local()
                    .or_else(|_| glib::DateTime::now_utc())
                    .expect("the clock can be read"),
                program: program.to_string(),
                arguments: arguments.to_vec(),
                state: RecordState::Running,
                revision,
            });
            while data.records.len() > MAX_RECORDS {
                data.records.pop_front();
            }
            id
        };
        self.changed();
        Entry {
            log: self.clone(),
            id,
            started: Instant::now(),
        }
    }

    /// A copy of the records, oldest first.
    pub fn snapshot(&self) -> Vec<Record> {
        self.data().records.iter().cloned().collect()
    }

    pub fn failed_total(&self) -> u64 {
        self.data().failed_total
    }

    /// Removes all records. Commands still running are not shown when they finish.
    pub fn clear(&self) {
        self.data().records.clear();
        self.changed();
    }

    /// Calls `listener` on the main thread after changes. Several changes in a
    /// row may lead to one call. Call this on the main thread.
    pub fn subscribe(&self, listener: impl Fn() + 'static) {
        LISTENERS.with(|listeners| listeners.borrow_mut().push(Box::new(listener)));
    }

    fn update(&self, id: u64, change: impl FnOnce(&mut RecordState)) {
        {
            let mut data = self.data();
            let revision = data.revision();
            let Some(record) = data.records.iter_mut().find(|r| r.id == id) else {
                return;
            };
            let was_failed = record.is_failed();
            change(&mut record.state);
            record.revision = revision;
            if record.is_failed() && !was_failed {
                data.failed_total += 1;
            }
        }
        self.changed();
    }

    fn changed(&self) {
        if self.inner.wake_pending.swap(true, Ordering::AcqRel) {
            return;
        }
        let inner = self.inner.clone();
        glib::idle_add_once(move || {
            inner.wake_pending.store(false, Ordering::Release);
            LISTENERS.with(|listeners| listeners.borrow().iter().for_each(|listener| listener()));
        });
    }
}

/// A command in the log that is still running.
pub struct Entry {
    log: CommandLog,
    id: u64,
    started: Instant,
}

impl Entry {
    pub fn finish(&self, exit_code: Option<i32>, stdout: &str, stderr: &str) {
        let finished = Finished {
            duration: self.started.elapsed(),
            exit_code,
            stdout: Stream::new(stdout),
            stderr: Stream::new(stderr),
            rejected: false,
        };
        self.log
            .update(self.id, |state| *state = RecordState::Finished(finished));
    }

    /// Marks a finished command as failed because the app did not accept its output.
    pub fn reject(&self) {
        self.log.update(self.id, |state| {
            if let RecordState::Finished(finished) = state {
                finished.rejected = true;
            }
        });
    }

    pub fn not_started(&self, error: &str) {
        let error = error.to_string();
        self.log
            .update(self.id, |state| *state = RecordState::NotStarted(error));
    }

    pub fn launched(&self) {
        self.log
            .update(self.id, |state| *state = RecordState::Launched);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn keeps_the_newest_records() {
        let log = CommandLog::default();
        for i in 0..MAX_RECORDS + 5 {
            log.start("adb", &[i.to_string()]);
        }
        let records = log.snapshot();
        assert_eq!(records.len(), MAX_RECORDS);
        assert_eq!(records[0].arguments, ["5"]);
        assert_eq!(
            records.last().unwrap().arguments,
            [(MAX_RECORDS + 4).to_string()]
        );
    }

    #[test]
    fn marks_failures_once() {
        let log = CommandLog::default();
        let ok = log.start("adb", &args(&["devices"]));
        ok.finish(Some(0), "List of devices attached\n", "");
        let exit = log.start("adb", &args(&["pull", "x"]));
        exit.finish(Some(1), "", "adb: error: failed to stat remote object\n");
        let rejected = log.start("adb", &args(&["connect", "1.2.3.4"]));
        rejected.finish(Some(0), "failed to connect\n", "");
        rejected.reject();
        rejected.reject();
        let missing = log.start("/nowhere/adb", &[]);
        missing.not_started("No such file or directory");
        let failed: Vec<bool> = log.snapshot().iter().map(Record::is_failed).collect();
        assert_eq!(failed, [false, true, true, true]);
        assert_eq!(log.failed_total(), 3);
        log.clear();
        assert!(log.snapshot().is_empty());
        assert_eq!(log.failed_total(), 3);
        // Finishing a cleared command changes nothing.
        exit.finish(Some(1), "", "");
        assert!(log.snapshot().is_empty());
    }

    #[test]
    fn revision_changes_with_the_record() {
        let log = CommandLog::default();
        let entry = log.start("adb", &[]);
        let before = log.snapshot()[0].revision;
        entry.finish(Some(0), "", "");
        assert!(log.snapshot()[0].revision > before);
    }

    #[test]
    fn cuts_long_output_at_a_character() {
        let text = format!("{}é", "a".repeat(MAX_STREAM_BYTES - 1));
        let stream = Stream::new(&text);
        assert_eq!(stream.text.len(), MAX_STREAM_BYTES - 1);
        assert_eq!(stream.dropped, 2);
        assert_eq!(Stream::new("short").dropped, 0);
    }

    #[test]
    fn shows_commands_like_a_shell() {
        let log = CommandLog::default();
        log.start(
            "/opt/sdk/platform-tools/adb",
            &args(&["-s", "emulator-5554", "shell", "input", "text", "'a b'"]),
        );
        log.start("adb", &args(&["install", "/home/me/My Apps/app.apk", ""]));
        let records = log.snapshot();
        assert_eq!(
            records[0].short_command(),
            "adb -s emulator-5554 shell input text 'a b'"
        );
        assert!(
            records[0]
                .full_command()
                .starts_with("/opt/sdk/platform-tools/adb -s")
        );
        assert_eq!(
            records[1].short_command(),
            "adb install '/home/me/My Apps/app.apk' ''"
        );
    }

    #[test]
    fn copies_as_text() {
        let log = CommandLog::default();
        let entry = log.start("adb", &args(&["forward", "tcp:8080", "tcp:80"]));
        entry.finish(Some(0), "8080\n", "");
        let text = log.snapshot()[0].to_text();
        assert!(text.contains("] $ adb forward tcp:8080 tcp:80\nexit 0, "));
        assert!(text.ends_with("stdout:\n8080\n"));
    }

    #[test]
    fn quotes_words_the_shell_would_change() {
        assert_eq!(display_word("/sdcard/shot[1].png"), "'/sdcard/shot[1].png'");
        assert_eq!(display_word("{a,b}"), "'{a,b}'");
        assert_eq!(display_word("'a' 'b'"), r"''\''a'\'' '\''b'\'''");
        assert_eq!(display_word("plain"), "plain");
        // Text the app already quoted for the device shell stays as it is.
        let quoted = crate::tools::shell_quote("it's here");
        assert_eq!(display_word(&quoted), quoted);
    }

    #[test]
    fn formats_durations() {
        assert_eq!(format_duration(Duration::from_millis(182)), "182 ms");
        assert_eq!(format_duration(Duration::from_millis(5012)), "5.0 s");
    }
}
