use minutils_common::fail_with_error;
use std::env;
use std::fs;
use std::process::ExitCode;
use std::io;

fn read_file(path: &str) -> ExitCode {
    match fs::File::open(path) {
        Ok(x) => {print_readbuf(io::BufReader::new(x), path); ExitCode::from(0)},
        Err(_) => fail_with_error(format!("Filesystem error on file '{}'. Exiting.", path).as_str(), 1),
    }
}


fn print_readbuf<READBUF: io::Read + io::BufRead>(readbuf: READBUF, path: &str) -> ExitCode {
    for line in readbuf.lines() {
        match line {
            Ok(x) => println!("{}", x),
            Err(_) => return fail_with_error(format!("Filesystem error on file '{}'. Exiting.", path).as_str(), 1),
        }
    }
    ExitCode::from(0)
}


fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if !args.is_empty() {
        for i in args {
            if read_file(&i) != ExitCode::SUCCESS {
                return ExitCode::from(1);
            }
        }
        ExitCode::SUCCESS
    } else {
        let stdin = io::stdin();
        let handle = stdin.lock();
        print_readbuf(handle, "stdin")
    }
}
