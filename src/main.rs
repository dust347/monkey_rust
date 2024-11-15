use std::env;
use std::io;
use std::io::Read;
use std::io::Write;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    match env::var("USER") {
        Ok(name) => println!("Hello {}! This is the monkey programming language!", name),
        Err(_) => println!("Hello!"),
    }

    repl::start(io::stdin().by_ref(), io::stdout().by_ref())
}
