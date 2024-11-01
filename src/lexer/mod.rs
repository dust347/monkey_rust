use crate::token::{self, Token};

pub struct Lexer {
    input: String,
    position: usize,      // 当前字符
    read_position: usize, // 当前字符之后的一个字符
    ch: Option<char>,     // 当前正在查看的字符
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: Option::None,
        };
        l.read_char();

        return l;
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position);

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let t: token::Token;
        self.skip_white_space();

        if let Some(ch) = self.ch {
            match ch {
                '=' => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        t = Token::new_eq();
                    } else {
                        t = Token::new(token::TokenType::Assign, ch);
                    }
                }
                '+' => t = Token::new(token::TokenType::Plus, ch),
                '-' => t = Token::new(token::TokenType::Minus, ch),
                '!' => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        t = Token::new_not_eq();
                    } else {
                        t = Token::new(token::TokenType::Bang, ch);
                    }
                }
                '/' => t = Token::new(token::TokenType::Slash, ch),
                '*' => t = Token::new(token::TokenType::Asterisk, ch),
                '<' => t = Token::new(token::TokenType::Lt, ch),
                '>' => t = Token::new(token::TokenType::Gt, ch),
                ';' => t = Token::new(token::TokenType::Semicolon, ch),
                ',' => t = Token::new(token::TokenType::Comma, ch),
                '(' => t = Token::new(token::TokenType::Lparen, ch),
                ')' => t = Token::new(token::TokenType::Rparen, ch),
                '{' => t = Token::new(token::TokenType::Lbrace, ch),
                '}' => t = Token::new(token::TokenType::Rbrace, ch),

                _ => {
                    if ch.is_alphabetic() || ch == '_' {
                        t = Token::new_ident(self.read_identifier());
                        return t;
                    } else if ch.is_numeric() {
                        t = Token::new_int(self.read_number());
                        return t;
                    } else {
                        t = token::Token::new(token::TokenType::Illegal, ch);
                    }
                }
            }
        } else {
            t = Token {
                typ: token::TokenType::EOF,
                literal: String::new(),
            }
        }
        self.read_char();
        t
    }

    fn read_identifier(&mut self) -> String {
        let postion = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }

        self.input
            .chars()
            .skip(postion)
            .take(self.position - postion)
            .collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while let Some(c) = self.ch {
            if !c.is_numeric() {
                break;
            }
            self.read_char();
        }

        self.input
            .chars()
            .skip(position)
            .take(self.position - position)
            .collect()
    }

    fn skip_white_space(&mut self) {
        while let Some(c) = self.ch {
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.read_char();
                continue;
            }
            break;
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }
}

fn is_letter(ch: Option<char>) -> bool {
    if let Some(c) = ch {
        return c.is_alphabetic() || c == '_';
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char() {
        let s = String::from("hello world");

        println!("count: {}", s.chars().count());
        if let Some(c) = s.chars().nth(0) {
            println!("1th: {}", c)
        } else {
            println!("none");
        }
    }

    #[test]
    fn test_next_token() {
        let input: &str = r"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
           x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        ";
        let expected_tokens = vec![
            Token {
                typ: token::TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                typ: token::TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("ten"),
            },
            Token {
                typ: token::TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("add"),
            },
            Token {
                typ: token::TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                typ: token::TokenType::Function,
                literal: String::from("fn"),
            },
            Token {
                typ: token::TokenType::Lparen,
                literal: String::from("("),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("x"),
            },
            Token {
                typ: token::TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("y"),
            },
            Token {
                typ: token::TokenType::Rparen,
                literal: String::from(")"),
            },
            Token {
                typ: token::TokenType::Lbrace,
                literal: String::from("{"),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("x"),
            },
            Token {
                typ: token::TokenType::Plus,
                literal: String::from("+"),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("y"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Rbrace,
                literal: String::from("}"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("result"),
            },
            Token {
                typ: token::TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("add"),
            },
            Token {
                typ: token::TokenType::Lparen,
                literal: String::from("("),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                typ: token::TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                typ: token::TokenType::Ident,
                literal: String::from("ten"),
            },
            Token {
                typ: token::TokenType::Rparen,
                literal: String::from(")"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Bang,
                literal: String::from("!"),
            },
            Token {
                typ: token::TokenType::Minus,
                literal: String::from("-"),
            },
            Token {
                typ: token::TokenType::Slash,
                literal: String::from("/"),
            },
            Token {
                typ: token::TokenType::Asterisk,
                literal: String::from("*"),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                typ: token::TokenType::Lt,
                literal: String::from("<"),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                typ: token::TokenType::Gt,
                literal: String::from(">"),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::If,
                literal: String::from("if"),
            },
            Token {
                typ: token::TokenType::Lparen,
                literal: String::from("("),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                typ: token::TokenType::Lt,
                literal: String::from("<"),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                typ: token::TokenType::Rparen,
                literal: String::from(")"),
            },
            Token {
                typ: token::TokenType::Lbrace,
                literal: String::from("{"),
            },
            Token {
                typ: token::TokenType::Return,
                literal: String::from("return"),
            },
            Token {
                typ: token::TokenType::True,
                literal: String::from("true"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Rbrace,
                literal: String::from("}"),
            },
            Token {
                typ: token::TokenType::Else,
                literal: String::from("else"),
            },
            Token {
                typ: token::TokenType::Lbrace,
                literal: String::from("{"),
            },
            Token {
                typ: token::TokenType::Return,
                literal: String::from("return"),
            },
            Token {
                typ: token::TokenType::False,
                literal: String::from("false"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Rbrace,
                literal: String::from("}"),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                typ: token::TokenType::Eq,
                literal: String::from("=="),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                typ: token::TokenType::NotEq,
                literal: String::from("!="),
            },
            Token {
                typ: token::TokenType::Int,
                literal: String::from("9"),
            },
            Token {
                typ: token::TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                typ: token::TokenType::EOF,
                literal: String::new(),
            },
        ];

        let mut l = Lexer::new(input);
        for t in expected_tokens.iter() {
            let tok = l.next_token();
            assert_eq!(tok.literal, t.literal);
            assert_eq!(tok.typ, t.typ);
        }
    }

    #[test]
    fn test_read_identifier() {
        let mut l = Lexer::new("hello_123+==");
        let s = l.read_identifier();
        println!("s: {}", s);
        assert_eq!(s, "hello_");
    }

    #[test]
    fn test_option() {
        let ch = Some('a');
        if let Some('a') = ch {
            println!("a == a");
        } else {
            println!("false")
        }
    }
}
