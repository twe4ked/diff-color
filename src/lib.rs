//! Color `diff --unified` output.
//!
//! # Example
//!
//! ```bash
//! $ diff --unified <(git show 2e60019:README.md) README.md | diff-color
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
