use minutils_common::fail_with_error;
use std::env;
use std::fs;
use std::process::ExitCode;

fn list_dir(path: &str) -> ExitCode {
    match fs::read_dir(path) {
        Ok(x) => {
            let mut result: Vec<_> = x.flatten().collect();
            result.sort_by_key(|y| y.file_name());
            for i in result {
                print!("{} ", i.file_name().to_string_lossy());
            }
            println!();
            ExitCode::from(0)
        }
        Err(_) => fail_with_error("Filesystem error.", 1),
    }
}
fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if !args.is_empty() {
        list_dir(&args[0])
    } else {
        list_dir(".")
    }
}
