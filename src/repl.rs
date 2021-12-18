use ansi_term::Colour::Fixed;
use std::io::{self, Write};

use super::core::lexer;

pub fn repl() {
    println!(
        "{}{}{}{}{} Interpreter",
        Fixed(9).bold().paint("B"),
        Fixed(11).bold().paint("A"),
        Fixed(10).bold().paint("S"),
        Fixed(12).bold().paint("I"),
        Fixed(13).bold().paint("C")
    );
    println!("version 0.0.0\n");

    let mut input = String::new();
    loop {
        print!("{} >> ", Fixed(14).paint("BASIC"));
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        input.pop();

        if input == "exit" {
            break;
        }

        match lexer::Lexer::init(input.clone()).tokenize() {
            Ok(tokens) => {
                println!("{:?}", tokens);
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }

        input = format!("");
    }
}
