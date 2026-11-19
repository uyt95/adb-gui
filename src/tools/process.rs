//! Running external commands and checking their output.

use std::process::{Command, Stdio};

use regex::RegexBuilder;

use super::Result;

/// Runs a command and returns its stdout, or stderr (falling back to stdout) on failure.
pub fn execute(command: &str, arguments: &[String]) -> Result<String> {
    let output = Command::new(command)
        .args(arguments)
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("{command}: {error}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    if output.status.success() {
        return Ok(stdout);
    }
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(if stderr.is_empty() {
        stdout.trim().to_string()
    } else {
        stderr
    })
}

/// Whether the output matches any of the patterns (multi-line, case-insensitive).
/// An empty pattern list accepts any output.
pub fn validate(output: &str, patterns: &[&str]) -> bool {
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
pub fn execute_validated(command: &str, arguments: &[String], patterns: &[&str]) -> Result<String> {
    let output = execute(command, arguments)?;
    if validate(&output, patterns) {
        Ok(output)
    } else {
        Err(output.trim().to_string())
    }
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
    }

    #[test]
    fn execute_reports_stderr() {
        let error = execute("sh", &["-c".into(), "echo oops >&2; exit 1".into()]).unwrap_err();
        assert_eq!(error, "oops");
        assert_eq!(
            execute("sh", &["-c".into(), "echo hi".into()]).unwrap(),
            "hi\n"
        );
    }
}
