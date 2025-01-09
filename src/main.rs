// #![deny(warnings)]

use std::io::BufRead;
use std::iter::Iterator;

mod sampling;

fn main() {
    // Read the user's command line arguments (if any)
    //
    //   0 arguments  = get a single random line
    //   1 argument k = get that number of lines
    //  >1 arguments  = error
    //
    let args: Vec<_> = std::env::args().collect();

    let k = match args.len() {
        1 => 1,
        2 => match args[1].parse::<usize>() {
            Ok(parsed_k) => parsed_k,
            Err(_) => {
                eprintln!("Usage: randline [k]");
                std::process::exit(1)
            }
        },
        _ => {
            eprintln!("Usage: randline [k]");
            std::process::exit(1)
        }
    };

    let lines = std::io::stdin().lock().lines().map(|line| match line {
        Ok(ln) => ln,
        Err(e) => {
            eprintln!("Unable to read from stdin: {:?}", e);
            std::process::exit(1)
        }
    });

    let sample = sampling::reservoir_sample(lines, k);

    for line in sample {
        println!("{}", line);
    }
}
