use std::env;
use std::io;

mod lexer;
mod repl;
mod token;

fn main() {
    match env::var("USER") {
        Ok(name) => println!("Hello {}! This is the monkey programming language!", name),
        Err(_) => println!("Hello!"),
    }
    repl::start(io::stdin(), io::stdout())
}
