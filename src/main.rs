#![deny(warnings)]

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
            Ok(parsed_k) if parsed_k > 0 => parsed_k,
            _ => {
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

#[cfg(test)]
mod cli_tests {
    use assert_cmd::Command;

    // Note: for the purposes of the CLI tests, I trust that the reservoir
    // sampling code works correctly -- that's tested separately.  I'm just
    // checking the CLI parses the options correctly, so all the input lines
    // are the same for easy assertions.

    // If you call `randline` without any arguments, it picks a single line.
    #[test]
    fn it_selects_a_single_line_if_no_arg() {
        Command::cargo_bin("randline")
            .unwrap()
            .write_stdin("a\na\na\na\na\na\n")
            .assert()
            .success()
            .stdout("a\n")
            .stderr("");
    }

    // If you pass an argument `k` and there are more lines than `k`,
    // it selects a subset of them.
    #[test]
    fn it_selects_k_lines_if_more_lines_than_k() {
        Command::cargo_bin("randline")
            .unwrap()
            .arg("2")
            .write_stdin("a\na\na\na\na\na\n")
            .assert()
            .success()
            .stdout("a\na\n")
            .stderr("");
    }

    // If you pass an argument `k` and there are that number of lines,
    // it selects all of them.
    #[test]
    fn it_selects_k_lines_if_equal_lines_to_k() {
        Command::cargo_bin("randline")
            .unwrap()
            .arg("2")
            .write_stdin("a\na\n")
            .assert()
            .success()
            .stdout("a\na\n")
            .stderr("");
    }

    // If you pass an argument `k` and there are less lines than `k`,
    // it selects all of them.
    #[test]
    fn it_selects_k_lines_if_less_lines_than_k() {
        Command::cargo_bin("randline")
            .unwrap()
            .arg("5")
            .write_stdin("a\na\n")
            .assert()
            .success()
            .stdout("a\na\n")
            .stderr("");
    }

    // Passing a non-integer argument is an error.
    #[test]
    fn it_fails_if_non_integer_argument() {
        Command::cargo_bin("randline")
            .unwrap()
            .arg("XXX")
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("Usage: randline [k]\n");
    }

    // Passing k=0 is an error.
    #[test]
    fn it_fails_if_k_equals_zero() {
        Command::cargo_bin("randline")
            .unwrap()
            .arg("0")
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("Usage: randline [k]\n");
    }

    // Passing k<0 is an error.
    #[test]
    fn it_fails_if_k_negative() {
        Command::cargo_bin("randline")
            .unwrap()
            .arg("-1")
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("Usage: randline [k]\n");
    }

    // Passing more than one argument is an error.
    #[test]
    fn it_fails_if_too_many_args() {
        Command::cargo_bin("randline")
            .unwrap()
            .args(&["1", "2", "3"])
            .assert()
            .failure()
            .code(1)
            .stdout("")
            .stderr("Usage: randline [k]\n");
    }
}
