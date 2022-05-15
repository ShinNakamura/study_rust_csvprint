use std::process;

fn main() {
    if let Err(e) = csvprint::run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
