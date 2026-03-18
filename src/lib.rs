use std::process::ExitCode;

/// Print an error to stderr and return an exit status.
pub fn fail_with_error(error: &str, code: u8) -> ExitCode {
    eprintln!("{}", error);
    ExitCode::from(code)
}