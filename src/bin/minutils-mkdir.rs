use std::env;
use std::fs;
use std::process::ExitCode;
use minutils_common::fail_with_error;

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if !args.is_empty() {
        match fs::create_dir_all(&args[0]) {
            Ok(_) => ExitCode::from(0),
            Err(_) => fail_with_error("Filesystem error.", 1),
        }
    } else {
        fail_with_error("Bad arguments.", 64)
    }
}
