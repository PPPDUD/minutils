use minutils_common::fail_with_error;
use std::env;
use std::fs;
use std::process::ExitCode;

fn remove(path: &str) -> ExitCode {
    match fs::remove_file(path) {
        Ok(_) => ExitCode::from(0),
        Err(_) => fail_with_error("Filesystem error.", 1),
    }
}

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if !args.is_empty() {
        for i in args {
            if remove(&i) != ExitCode::SUCCESS {
                return ExitCode::from(1);
            }
            println!("Deleted {}.", i);
        }
        ExitCode::SUCCESS
    } else {
        fail_with_error("Bad arguments.", 64)
    }
}
