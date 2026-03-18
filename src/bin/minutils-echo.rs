use std::env;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if !args.is_empty() {
        println!("{}", args.join(" "));
    }
    else {
        println!();
    }
}
