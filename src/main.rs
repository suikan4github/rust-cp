// Copy the Standard input stream to the Standard output stream.

use std::io::{stdin, BufRead, BufReader};

fn main() {
    // Create a reader variable, tied with the standard input.
    let reader = BufReader::new(stdin());

    // For each lines of the standard input:
    for line in reader.lines() {
        // print that line to the standard output.
        // line iterator returns the Result type. So, wo need to unwrap.
        println!("{}", line.unwrap());
    }
}
