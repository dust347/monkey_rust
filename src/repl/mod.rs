use std::io::{self, BufRead};

use crate::{
    lexer::{self},
    token::TokenType,
};

pub fn start<I: io::Read, O: io::Write>(input: I, mut output: O) {
    let mut scanner = io::BufReader::new(input);

    loop {
        print!(">>>");

        let mut line = String::new();
        if let Err(err) = scanner.read_line(&mut line) {
            println!("err: {}", err.to_string());
            return;
        }

        let mut l = lexer::Lexer::new(&line);

        loop {
            let t = l.next_token();
            if let TokenType::EOF = t.typ {
                break;
            }

            // TODO 写
            writeln!(&mut output, "{:#?}", t).expect("write to output error");
        }
    }
}
