//! Color `diff --unified` output.
//!
//! # Example
//!
//! ```bash
//! $ diff --unified <(git show 2e60019:README.md) README.md | diff-color
//! ```
//!
//! # Installation
//!
//! ```bash
//! $ cargo install diff-color
//! ```
use colored::Colorize;
use std::io::{stdin, BufRead};
use std::process;

fn main() {
    for line in stdin().lock().lines() {
        let line = line.unwrap_or_else(|err| {
            println!("An error occured: {}", err);
            process::exit(1);
        });

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
}
