//! Color `diff --unified` output.
//!
//! ```bash
//! $ diff file_1 file_2 | diff-color
//! ```
use colored::Colorize;
use std::io::{stdin, BufRead, Error};
use std::result::Result;

pub fn run() -> Result<(), Error> {
    for line in stdin().lock().lines() {
        let line = line?;

        if line.starts_with("---") {
            println!("{}", line.yellow());
        } else if line.starts_with("+++") {
            println!("{}", line.yellow());
        } else if line.starts_with("@@ ") {
            println!("{}", line.cyan());
        } else if line.starts_with("+") {
            println!("{}", line.green());
        } else if line.starts_with("-") {
            println!("{}", line.red());
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
