use std::io::{self, BufRead};

use crate::parser::{self, Parser};
use crate::{
    lexer::{self},
    token::TokenType,
};

pub fn start<I: io::Read, O: io::Write>(input: &mut I, output: &mut O) {
    let mut scanner = io::BufReader::new(input);

    loop {
        print!(">>>");
        let mut line = String::new();
        if let Err(err) = scanner.read_line(&mut line) {
            println!("err: {}", err.to_string());
            return;
        }

        let mut l = lexer::Lexer::new(&line);
        let mut p = parser::Parser::new(&mut l);

        let program = p.parse_program();
        if p.errors().len() != 0 {
            print_parser_errors(output, p.errors());
            continue;
        }

        writeln!(output, "{}", program.to_string().as_str()).expect("write to output error");
    }
}

fn print_parser_errors(out: &mut impl io::Write, errors: Vec<String>) {
    for msg in errors.iter() {
        writeln!(out, "\t {}", msg.as_str()).expect("write to output error")
    }
}
