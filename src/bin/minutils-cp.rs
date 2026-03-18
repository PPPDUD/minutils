use std::env;
use std::fs;
use std::process::ExitCode;
use minutils_common::fail_with_error;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 3 {
        match fs::copy(&args[1], &args[2]) {
            Ok(_) => ExitCode::from(0),
            Err(_) => fail_with_error("Filesystem error.", 1)
        }
    }
    else {
        fail_with_error("Bad arguments.", 64)
    }
}
