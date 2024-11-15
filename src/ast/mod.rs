use crate::token::Token;

pub trait Node: ToString {
    fn token_literal(&self) -> &str;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl ToString for Program {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for stmt in self.statements.iter() {
            s.push_str(stmt.to_string().as_str());
        }

        s
    }
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        ""
    }
}

#[derive(Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(t: &Token, v: &str) -> Identifier {
        Identifier {
            token: t.clone(),
            value: String::from(v),
        }
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

#[derive(Default)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(t: &Token, i: i64) -> IntegerLiteral {
        IntegerLiteral {
            token: t.clone(),
            value: i,
        }
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

#[derive(Default)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push('(');
        s.push_str(&self.operator);
        if let Some(e) = self.right.as_ref() {
            s.push_str(e.to_string().as_str());
        }
        s.push(')');

        s
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

#[derive(Default)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str(self.token_literal());
        s.push_str(" ");
        s.push_str(self.name.to_string().as_str());
        s.push_str(" = ");
        if let Some(e) = self.value.as_ref() {
            s.push_str(e.as_ref().to_string().as_str());
        }
        s.push_str(";");

        s
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[derive(Default)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str(self.token_literal());
        s.push_str(" ");
        if let Some(e) = self.return_value.as_ref() {
            s.push_str(e.to_string().as_str());
        }
        s.push_str(";");

        s
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        let mut s = String::new();
        if let Some(e) = self.expression.as_ref() {
            s.push_str(e.to_string().as_str());
        }
        s
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

#[derive(Default)]
pub struct BlockStatement {
    pub token: Token, // { 词法单元
    pub statements: Vec<Box<dyn Statement>>,
}

impl ToString for BlockStatement {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for st in self.statements.iter() {
            s.push_str(st.to_string().as_str());
        }

        s
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
}

#[derive(Default)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Option<Box<dyn Expression>>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push('(');
        if let Some(e) = self.left.as_ref() {
            s.push_str(e.to_string().as_str());
        }

        s.push(' ');
        s.push_str(self.operator.as_str());
        s.push(' ');

        if let Some(e) = self.right.as_ref() {
            s.push_str(e.to_string().as_str());
        }
        s.push(')');

        s
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

#[derive(Default)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl ToString for Boolean {
    fn to_string(&self) -> String {
        self.token.literal.clone()
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Expression for Boolean {
    fn expression_node(&self) {}
}

#[derive(Default)]
pub struct IfExpression {
    pub token: Token, // if 语法单元
    pub condition: Option<Box<dyn Expression>>,
    pub consequence: Option<Box<BlockStatement>>,
    pub alternative: Option<Box<BlockStatement>>,
}

impl ToString for IfExpression {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str("if");
        if let Some(c) = self.condition.as_ref() {
            s.push_str(c.to_string().as_str());
        }
        s.push_str(" ");

        if let Some(c) = self.consequence.as_ref() {
            s.push_str(c.to_string().as_str());
        }

        if let Some(a) = self.alternative.as_ref() {
            s.push_str("else");
            s.push_str(a.to_string().as_str());
        }

        s
    }
}

impl Node for IfExpression {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Expression for IfExpression {
    fn expression_node(&self) {}
}

#[derive(Default)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Option<Box<BlockStatement>>,
}

impl ToString for FunctionLiteral {
    fn to_string(&self) -> String {
        let mut s = String::new();

        let mut params: Vec<String> = vec![];
        for p in self.parameters.iter() {
            params.push(p.to_string().clone());
        }

        s.push_str(self.token_literal());
        s.push_str("(");
        s.push_str(params.join(", ").as_str());
        s.push_str(")");
        if let Some(body) = self.body.as_ref() {
            s.push_str(body.to_string().as_str());
        }

        s
    }
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

#[derive(Default)]
pub struct CallExpression {
    pub token: Token,
    pub function: Option<Box<dyn Expression>>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl ToString for CallExpression {
    fn to_string(&self) -> String {
        let mut s = String::new();

        let mut args: Vec<String> = vec![];
        for a in self.arguments.iter() {
            args.push(a.to_string().clone());
        }

        if let Some(f) = self.function.as_ref() {
            s.push_str(f.to_string().as_str());
        }
        s.push_str("(");
        s.push_str(args.join(", ").as_str());
        s.push_str(")");

        s
    }
}

impl Node for CallExpression {
    fn token_literal(&self) -> &str {
        self.token.literal.as_str()
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}
}

mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_to_string() {
        let program = Program {
            statements: vec![Box::new(LetStatement {
                token: Token {
                    typ: TokenType::Let,
                    literal: String::from("let"),
                },
                name: Identifier {
                    token: Token {
                        typ: TokenType::Ident,
                        literal: String::from("myVar"),
                    },
                    value: String::from("myVal"),
                },
                value: Some(Box::new(Identifier {
                    token: Token {
                        typ: TokenType::Ident,
                        literal: String::from("anotherVal"),
                    },
                    value: String::from("anotherVar"),
                })),
            })],
        };

        assert_eq!("let myVal = anotherVar;", program.to_string().as_str());
        println!("{}", program.to_string());
    }
}
