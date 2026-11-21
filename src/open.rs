//! Open an artifact with the host's system opener.
//!
//! The target is always passed as one argument, never through a shell: on Windows `cmd /C
//! start` would read `&` in a URL as a command separator, so the URL handler is called
//! directly. Starting the opener is fire-and-forget, with its output discarded so it cannot
//! paint over the board: a failure becomes a status message, never a crash.

use std::process::{Command, Stdio};

/// Launch the system opener for `target`, returning a short reason on failure.
pub fn open_target(target: &str) -> Result<(), String> {
    let (program, args): (&str, Vec<&str>) = if cfg!(windows) {
        ("rundll32", vec!["url.dll,FileProtocolHandler", target])
    } else if cfg!(target_os = "macos") {
        ("open", vec![target])
    } else {
        ("xdg-open", vec![target])
    };
    Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("cannot open {target}: {error}"))
}
