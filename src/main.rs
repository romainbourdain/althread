use std::{fs, process::exit};

use althread_with_pest::{
    args::{cmd, Config},
    run,
};

fn main() {
    // Parse args and check input file
    let matches = cmd().get_matches();
    let config = Config::from_args(&matches);

    // Read file
    let buf = fs::read_to_string(&config.input).expect("Cannot read file");

    // Run code
    if let Err(e) = run(&buf) {
        e.report(&buf);
        exit(1);
    }
}
