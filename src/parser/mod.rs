use std::{collections::HashMap, f64::NAN, fmt::format};

use crate::{
    ast::{self, Expression, InfixExpression, IntegerLiteral},
    lexer,
    token::{self, TokenType},
};

type prefix_parse_fn = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type infix_parse_fn = fn(&mut Parser, Option<Box<dyn Expression>>) -> Option<Box<dyn Expression>>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    _Int,
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -x or !x
    Call,        // myFunction(x)
}

// tokenType => precedence
fn precedences(t: TokenType) -> Precedence {
    match t {
        TokenType::Eq | TokenType::NotEq => Precedence::Equals,
        TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
        TokenType::Plus | TokenType::Minus => Precedence::Sum,
        TokenType::Slash | TokenType::Asterisk => Precedence::Product,
        TokenType::Lparen => Precedence::Call,
        _ => Precedence::Lowest,
    }
}

pub struct Parser<'a> {
    l: &'a mut lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,

    errors: Vec<String>,

    prefix_parse_fns: HashMap<TokenType, prefix_parse_fn>,
    infix_parse_fns: HashMap<TokenType, infix_parse_fn>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut lexer::Lexer) -> Parser {
        let mut p = Parser {
            l: l,
            cur_token: token::Token::new(token::TokenType::EOF, ' '),
            peek_token: token::Token::new(token::TokenType::EOF, ' '),
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // 注册 prefix 函数
        p.register_prefix(TokenType::Ident, parse_identifier);
        p.register_prefix(TokenType::Int, parse_integer_literal);
        p.register_prefix(TokenType::Bang, parse_prefix_expression);
        p.register_prefix(TokenType::Minus, parse_prefix_expression);
        p.register_prefix(TokenType::True, parse_boolean);
        p.register_prefix(TokenType::False, parse_boolean);
        p.register_prefix(TokenType::Lparen, parse_grouped_expression);
        p.register_prefix(TokenType::If, parse_if_expression);
        p.register_prefix(TokenType::Function, parse_function_literal);

        // 注册 infix 函数
        p.register_infix(TokenType::Plus, parse_infix_expression);
        p.register_infix(TokenType::Minus, parse_infix_expression);
        p.register_infix(TokenType::Slash, parse_infix_expression);
        p.register_infix(TokenType::Asterisk, parse_infix_expression);
        p.register_infix(TokenType::Eq, parse_infix_expression);
        p.register_infix(TokenType::NotEq, parse_infix_expression);
        p.register_infix(TokenType::Lt, parse_infix_expression);
        p.register_infix(TokenType::Gt, parse_infix_expression);
        p.register_infix(TokenType::Lparen, parse_call_expression);

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn cur_token_is(&self, t: token::TokenType) -> bool {
        self.cur_token.typ == t
    }

    fn peek_token_is(&self, t: token::TokenType) -> bool {
        self.peek_token.typ == t
    }

    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        }

        self.peek_error(t);
        false
    }

    fn parse_let_statement(&mut self) -> Option<Box<ast::LetStatement>> {
        let mut stmt = Box::new(ast::LetStatement::default());
        stmt.token = self.cur_token.clone();

        if !self.expect_peek(token::TokenType::Ident) {
            return None;
        }

        stmt.name = ast::Identifier::new(&self.cur_token, &self.cur_token.literal);
        if !self.expect_peek(token::TokenType::Assign) {
            return None;
        }

        self.next_token();

        stmt.value = self.parse_expression(Precedence::Lowest);

        if !self.peek_token_is(token::TokenType::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_return_statement(&mut self) -> Box<ast::ReturnStatement> {
        let mut stmt = Box::new(ast::ReturnStatement::default());
        stmt.token = self.cur_token.clone();
        self.next_token();

        stmt.return_value = self.parse_expression(Precedence::Lowest);

        if !self.peek_token_is(token::TokenType::Semicolon) {
            self.next_token();
        }

        stmt
    }

    fn parse_expression_statement(&mut self) -> Box<ast::ExpressionStatement> {
        let mut stmt = Box::new(ast::ExpressionStatement {
            token: self.cur_token.clone(),
            expression: None,
        });
        stmt.expression = self.parse_expression(Precedence::Lowest);

        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        stmt
    }

    fn parse_expression(&mut self, op: Precedence) -> Option<Box<dyn ast::Expression>> {
        let prefix = self.prefix_parse_fns.get(&self.cur_token.typ);

        match prefix {
            Some(prefix) => {
                let mut leftExp = prefix(self);
                while !self.peek_token_is(TokenType::Semicolon) && op < self.peek_precedence() {
                    let infix = self.infix_parse_fns.get(&self.peek_token.typ).cloned();

                    if let Some(infix) = infix {
                        self.next_token();
                        leftExp = infix(self, leftExp);
                    } else {
                        return leftExp;
                    }
                }

                leftExp
            }
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.typ);
                None
            }
        }
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.typ {
            token::TokenType::Let => {
                if let Some(s) = self.parse_let_statement() {
                    let b: Box<dyn ast::Statement> = s;
                    return Some(b);
                }
                None
            }

            token::TokenType::Return => {
                return Some(self.parse_return_statement());
            }
            _ => Some(self.parse_expression_statement()),
        }
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::default();
        while self.cur_token.typ != token::TokenType::EOF {
            if let Some(s) = self.parse_statement() {
                program.statements.push(s);
            }
            self.next_token();
        }

        program
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, t: token::TokenType) {
        self.errors.push(format!(
            "expected next token to  be {:?}, got {:?} instead",
            t, self.peek_token.typ
        ))
    }

    fn register_prefix(&mut self, t: TokenType, f: prefix_parse_fn) {
        self.prefix_parse_fns.insert(t, f);
    }

    fn register_infix(&mut self, t: TokenType, f: infix_parse_fn) {
        self.infix_parse_fns.insert(t, f);
    }

    fn no_prefix_parse_fn_error(&mut self, t: TokenType) {
        self.errors
            .push(format!("no prefix parse function for {:?} found", t))
    }

    fn peek_precedence(&self) -> Precedence {
        precedences(self.peek_token.typ)
    }

    fn cur_precedence(&self) -> Precedence {
        precedences(self.cur_token.typ)
    }

    fn parse_block_statement(&mut self) -> Option<Box<ast::BlockStatement>> {
        let mut block = Box::new(ast::BlockStatement::default());
        block.token = self.cur_token.clone();

        self.next_token();

        while !self.cur_token_is(TokenType::Rbrace) && !self.cur_token_is(TokenType::EOF) {
            if let Some(stmt) = self.parse_statement() {
                block.statements.push(stmt);
            }
            self.next_token()
        }

        Some(block)
    }

    fn parse_function_parameters(&mut self) -> Vec<ast::Identifier> {
        let mut identifiers: Vec<ast::Identifier> = vec![];

        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        identifiers.push(ast::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(ast::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }

        if !self.expect_peek(TokenType::Rparen) {
            return vec![];
        }

        identifiers
    }

    fn parse_call_arguments(&mut self) -> Vec<Box<dyn Expression>> {
        let mut args: Vec<Box<dyn Expression>> = vec![];

        if self.peek_token_is(TokenType::Rparen) {
            self.next_token();
            return args;
        }

        self.next_token();
        if let Some(e) = self.parse_expression(Precedence::Lowest) {
            args.push(e)
        }

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            if let Some(e) = self.parse_expression(Precedence::Lowest) {
                args.push(e)
            }
        }

        if !self.expect_peek(TokenType::Rparen) {
            return vec![];
        }

        args
    }
}

fn parse_identifier(p: &mut Parser) -> Option<Box<dyn Expression>> {
    Some(Box::new(ast::Identifier::new(
        &p.cur_token,
        p.cur_token.literal.as_str(),
    )))
}

fn parse_integer_literal(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut l = Box::new(IntegerLiteral::new(&p.cur_token, 0));

    let v: Result<i64, _> = p.cur_token.literal.as_str().parse();
    match v {
        Ok(num) => {
            l.value = num;
            Some(l)
        }

        Err(_) => {
            let msg = format!(
                "could not parse {} as integer",
                p.cur_token.literal.as_str()
            );
            p.errors.push(msg);

            None
        }
    }
}

fn parse_prefix_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut expression = Box::new(ast::PrefixExpression {
        token: p.cur_token.clone(),
        operator: p.cur_token.literal.clone(),
        right: None,
    });

    p.next_token();

    expression.right = p.parse_expression(Precedence::Prefix);
    Some(expression)
}

fn parse_boolean(p: &mut Parser) -> Option<Box<dyn Expression>> {
    Some(Box::new(ast::Boolean {
        token: p.cur_token.clone(),
        value: p.cur_token_is(TokenType::True),
    }))
}

fn parse_grouped_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    p.next_token();

    let exp = p.parse_expression(Precedence::Lowest);

    if !p.expect_peek(TokenType::Rparen) {
        return None;
    }

    exp
}

fn parse_if_expression(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut expression = Box::new(ast::IfExpression::default());
    expression.token = p.cur_token.clone();

    if !p.expect_peek(TokenType::Lparen) {
        return None;
    }

    p.next_token();
    expression.condition = p.parse_expression(Precedence::Lowest);

    if !p.expect_peek(TokenType::Rparen) {
        return None;
    }

    if !p.expect_peek(TokenType::Lbrace) {
        return None;
    }

    expression.consequence = p.parse_block_statement();

    // else
    if p.peek_token_is(TokenType::Else) {
        p.next_token();

        if !p.expect_peek(TokenType::Lbrace) {
            return None;
        }

        expression.alternative = p.parse_block_statement();
    }

    Some(expression)
}

fn parse_function_literal(p: &mut Parser) -> Option<Box<dyn Expression>> {
    let mut lit = Box::new(ast::FunctionLiteral::default());
    lit.token = p.cur_token.clone();

    if !p.expect_peek(TokenType::Lparen) {
        return None;
    }

    lit.parameters = p.parse_function_parameters();

    if !p.expect_peek(TokenType::Lbrace) {
        return None;
    }

    lit.body = p.parse_block_statement();

    Some(lit)
}

fn parse_infix_expression(
    p: &mut Parser,
    left: Option<Box<dyn Expression>>,
) -> Option<Box<dyn ast::Expression>> {
    let mut expression = Box::new(InfixExpression {
        token: p.cur_token.clone(),
        operator: p.cur_token.literal.clone(),
        left: left,
        right: None,
    });

    let precedence = p.cur_precedence();
    p.next_token();
    expression.right = p.parse_expression(precedence);

    Some(expression)
}

fn parse_call_expression(
    p: &mut Parser,
    function: Option<Box<dyn ast::Expression>>,
) -> Option<Box<dyn ast::Expression>> {
    let mut exp = ast::CallExpression::default();
    exp.arguments = p.parse_call_arguments();
    exp.function = function;

    Some(Box::new(exp))
}

mod tests {

    use self::{ast::Program, lexer::Lexer};

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r"
            let x = 5;
            let y = 10;
            let foobar = 838383;
            ";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);

        let mut program = p.parse_program();
        assert_eq!(program.statements.len(), 3);

        let expected = vec!["x", "y", "foobar"];
        for i in 0..expected.len() {
            let stmt = program.statements[i].as_ref();
            assert_eq!(stmt.token_literal(), "let")
            // TODO 不太会 rust 反射，这里暂时省略
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar";

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        assert_eq!(1, program.statements.len());
        println!("len: {}", program.statements.len());

        for s in program.statements.iter() {
            println!("statement {}", s.to_string())
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();

        assert_eq!(1, program.statements.len());
        println!("len: {}", program.statements.len());

        for s in program.statements.iter() {
            println!("statement {}", s.to_string())
        }
    }

    fn check_parse_errors(p: &Parser) {
        let errs = p.errors();
        for err in errs.iter() {
            println!("err: {}", err.as_str())
        }
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let input = "!5;";

        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        check_parse_errors(&p);

        assert_eq!(1, program.statements.len());
        println!("len: {}", program.statements.len());

        for s in program.statements.iter() {
            println!("statement {}", s.to_string())
        }
    }

    fn add(i: i32, j: i32) -> i32 {
        i + j
    }

    type new_fn = fn(i32, i32) -> i32;

    #[test]
    fn test_fn() {
        let f = add;
        println!("{}", f(1, 3));
    }

    #[test]
    fn test_ord() {
        println!("{}", Precedence::_Int > Precedence::Equals);
        println!("{}", Precedence::Call > Precedence::Prefix);
    }

    struct Expected<'a> {
        input: &'a str,
        expected: &'a str,
    }

    impl<'a> Expected<'a> {
        fn new(input: &'a str, expected: &'a str) -> Expected<'a> {
            Expected {
                input: input,
                expected: expected,
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let expected_vec: Vec<Expected> = vec![
            Expected::new("-a * b", "((-a) * b)"),
            Expected::new("!-a", "(!(-a))"),
            Expected::new("a + b + c", "((a + b) + c)"),
            Expected::new("a + b - c", "((a + b) - c)"),
            Expected::new("a * b * c", "((a * b) * c)"),
            Expected::new("a * b / c", "((a * b) / c)"),
            Expected::new("a + b / c", "(a + (b / c))"),
            Expected::new("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            Expected::new("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            Expected::new("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            Expected::new("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            Expected::new(
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            Expected::new("true", "true"),
            Expected::new("false", "false"),
            Expected::new("3 > 5 == false", "((3 > 5) == false)"),
            Expected::new("3 < 5 == true", "((3 < 5) == true)"),
            Expected::new("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            Expected::new("(5 + 5) * 2", "((5 + 5) * 2)"),
            Expected::new("2 / (5 + 5)", "(2 / (5 + 5))"),
            Expected::new("-(5 + 5)", "(-(5 + 5))"),
            Expected::new("!(true == true)", "(!(true == true))"),
            Expected::new("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            Expected::new(
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            Expected::new(
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            Expected::new("fn(x, y) { x + y; }", "fn(x, y)(x + y)"),
            Expected::new("if (x < y) { x }", "if(x < y) x"),
        ];

        for tt in expected_vec.iter() {
            let mut l = Lexer::new(tt.input);
            let mut p = Parser::new(&mut l);

            let program = p.parse_program();
            check_parse_errors(&p);

            assert_eq!(tt.expected, program.to_string().as_str());
        }
    }
}
