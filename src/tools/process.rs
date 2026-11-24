//! Running external commands and checking their output.

use std::process::{Command, Stdio};

use regex::RegexBuilder;

use super::Result;
use super::log::CommandLog;

/// Runs a command and returns its stdout, or stderr (falling back to stdout) on failure.
pub fn execute(log: &CommandLog, command: &str, arguments: &[String]) -> Result<String> {
    execute_checked(log, command, arguments, |_| true)
}

/// Like [`execute`], and fails with the output when `accept` rejects it. The
/// command is logged as failed then too.
pub fn execute_checked(
    log: &CommandLog,
    command: &str,
    arguments: &[String],
    accept: impl Fn(&str) -> bool,
) -> Result<String> {
    let entry = log.start(command, arguments);
    let output = match Command::new(command)
        .args(arguments)
        .stdin(Stdio::null())
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            let message = format!("{command}: {error}");
            entry.not_started(&message);
            return Err(message);
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr);
    entry.finish(output.status.code(), &stdout, &stderr);
    if !output.status.success() {
        let stderr = stderr.trim();
        return Err(if stderr.is_empty() {
            stdout.trim().to_string()
        } else {
            stderr.to_string()
        });
    }
    if !accept(&stdout) {
        entry.reject();
        return Err(stdout.trim().to_string());
    }
    Ok(stdout)
}

/// Whether the output matches any of the patterns (multi-line, case-insensitive).
/// An empty pattern list accepts any output.
///
/// Surrounding whitespace is trimmed first. adb output ends with a newline, and in
/// multi-line mode `^$` would otherwise match the empty spot after it, so an error
/// message would pass a check meant for "no output". Patterns that must cover the
/// whole output use `\A` and `\z`, since `^` and `$` match at every line.
pub fn validate(output: &str, patterns: &[&str]) -> bool {
    let output = output.trim();
    patterns.is_empty()
        || patterns.iter().any(|pattern| {
            RegexBuilder::new(pattern)
                .multi_line(true)
                .case_insensitive(true)
                .build()
                .map(|regex| regex.is_match(output))
                .unwrap_or(false)
        })
}

/// Runs a command and fails with its output when the output does not validate.
pub fn execute_validated(
    log: &CommandLog,
    command: &str,
    arguments: &[String],
    patterns: &[&str],
) -> Result<String> {
    execute_checked(log, command, arguments, |output| validate(output, patterns))
}

/// Finds an executable on `PATH`, like `which`.
pub fn find_executable(name: &str) -> Option<String> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|candidate| {
            use std::os::unix::fs::PermissionsExt;
            candidate
                .metadata()
                .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
        .map(|p| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_like_the_flutter_app() {
        assert!(validate("", &["^$"]));
        assert!(validate(
            "connected to 1.2.3.4:5555\n",
            &["(already )?connected to .*"]
        ));
        assert!(validate(
            "Performing Streamed Install\nSuccess\n",
            &[".*success$"]
        ));
        assert!(!validate(
            "failed to connect\n",
            &["(already )?connected to .*"]
        ));
        assert!(validate("anything", &[]));
    }

    #[test]
    fn pull_validation_rejects_adb_errors() {
        let pattern = ["^(?:adb: error:)?$|^[^a]|^a[^d]"];
        assert!(validate("/sdcard/x.png: 1 file pulled.\n", &pattern));
        assert!(!validate(
            "adb: error: failed to stat remote object",
            &pattern
        ));
        assert!(!validate(
            "adb: error: failed to stat remote object\n",
            &pattern
        ));
    }

    #[test]
    fn empty_check_rejects_output_with_trailing_newline() {
        assert!(validate("\n", &["^$"]));
        assert!(!validate("Error: Unknown keycode\n", &["^$"]));
        assert!(validate("8080\n", &[r"^\s*8080\s*$"]));
    }

    #[test]
    fn execute_reports_stderr() {
        let log = CommandLog::default();
        let error =
            execute(&log, "sh", &["-c".into(), "echo oops >&2; exit 1".into()]).unwrap_err();
        assert_eq!(error, "oops");
        assert_eq!(
            execute(&log, "sh", &["-c".into(), "echo hi".into()]).unwrap(),
            "hi\n"
        );
        let records = log.snapshot();
        assert!(records[0].is_failed());
        assert!(!records[1].is_failed());
    }

    #[test]
    fn rejected_output_is_logged_as_failed() {
        let log = CommandLog::default();
        let error = execute_validated(&log, "sh", &["-c".into(), "echo nope".into()], &["^ok$"])
            .unwrap_err();
        assert_eq!(error, "nope");
        assert!(execute(&log, "/nonexistent/adb", &[]).is_err());
        let records = log.snapshot();
        assert!(records.iter().all(|r| r.is_failed()));
        assert!(matches!(
            records[1].state,
            super::super::log::RecordState::NotStarted(_)
        ));
    }
}
