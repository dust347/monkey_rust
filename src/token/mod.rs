#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    EOF,
    // 标识符
    Ident,
    Int,
    // 运算符
    Assign,   // =
    Plus,     // +
    Minus,    // -
    Bang,     // !
    Asterisk, // *
    Slash,    // /

    Lt,    // <
    Gt,    // >
    Eq,    // ==
    NotEq, // !=

    // 分割符
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // 关键字
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

fn look_up_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(typ: TokenType, ch: char) -> Token {
        Token {
            typ: typ,
            literal: String::from(ch),
        }
    }

    pub fn new_ident(s: String) -> Token {
        Token {
            typ: look_up_ident(&s),
            literal: s,
        }
    }

    pub fn new_eq() -> Token {
        Token {
            typ: TokenType::Eq,
            literal: String::from("=="),
        }
    }

    pub fn new_not_eq() -> Token {
        Token {
            typ: TokenType::NotEq,
            literal: String::from("!="),
        }
    }

    pub fn new_int(s: String) -> Token {
        Token {
            typ: TokenType::Int,
            literal: s,
        }
    }
}

#[cfg(test)]
mod tests {}
