use std::error::Error;
use std::io::{stdin, stdout, Write};

use crate::lexer::Lexer;

const PROMPT: &'static str = ">> ";

pub fn start() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    loop {
        print!("{}", PROMPT);
        stdout().flush().unwrap();

        stdin().read_line(&mut line)?;
        let mut lexer = Lexer::new(&line);
        while let Some(tok) = lexer.next_token() {
            println!("{:?}", tok);
        }
    }
}
