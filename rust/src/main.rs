use std::error::Error;

use clap::Parser;
use monkey::repl;

#[derive(Parser)]
struct Monkey {}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello! This is the Monkey programming language!\n");
    repl::start()
}
