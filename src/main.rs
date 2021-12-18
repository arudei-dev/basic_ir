pub mod core;
pub mod repl;

// use std::fs::read_to_string;

fn main() {
    // let content = read_to_string("./sample.bas").unwrap();

    // lex(content);
    repl::repl();
}
