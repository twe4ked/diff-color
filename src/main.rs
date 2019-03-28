use diff_color;
use std::process;

fn main() {
    diff_color::run().unwrap_or_else(|err| {
        println!("An error occured: {}", err);
        process::exit(1);
    });
}
