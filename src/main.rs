#![deny(warnings)]

use std::iter::Iterator;

mod sampling;

fn main() {
    // Read the user's command line arguments (if any)
    //
    //   0 arguments  = get a single random line
    //   1 argument N = get that number of lines
    //  >1 arguments  = error
    //
    let args: Vec<_> = std::env::args().collect();

    let n = match args.len() {
        1 => 0,
        2 => match args[1].parse::<usize>() {
            Ok(parsed_n) => parsed_n,
            Err(_) => {
                eprintln!("Usage: randline [N]");
                std::process::exit(1)
            }
        },
        _ => {
            eprintln!("Usage: randline [N]");
            std::process::exit(1)
        }
    };

    // Read the first N lines from stdout
    // let stdin = io::stdin();

    let a = [1, 2, 3];
    let iter = a.iter();

    println!("{:?}", sampling::reservoir_sample(iter, n));

    println!("n = {:?}", n);
}
