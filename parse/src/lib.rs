use ast::{Program, Statement, Type};
use lex::{tokens::Token, Lexer};

mod branch;
mod expression;
mod function;
mod loop_;
mod variable;

// entry
pub fn parse(input: Lexer) -> Result<Program, String> {
    let mut parser = Parser::new(input);
    parser.parse()
}

struct LVar {
    name: String,
    offset: usize,
    type_: Type,
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Precedence {
    Lowest,
    Assignment,
    Equals,
    LessGreater,
    Sum,
    Product,
    LBracket,
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peeked_token: Token,
    locals: Vec<LVar>,
}

/// parser base
impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next();
        let peeked_token = lexer.next();
        Self {
            lexer,
            current_token,
            peeked_token,
            locals: Vec::new(),
        }
    }

    fn peek_precedence(&self) -> Precedence {
        self.get_precedence(self.peeked_token.clone())
    }

    fn get_precedence(&self, token: Token) -> Precedence {
        match token {
            Token::Assignment => Precedence::Assignment,
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::Lt | Token::LtEq | Token::Gt | Token::GtEq => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::LBracket => Precedence::LBracket,
            _ => Precedence::Lowest,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peeked_token.clone();
        self.peeked_token = self.lexer.next();
    }
}

impl Parser {
    fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while self.current_token != Token::Eof {
            statements.push(self.parse_statement()?);
            self.next_token();
        }
        Ok(Program::new(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token {
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::For => self.parse_for_statement(),
            Token::Return => self.parse_return_statement(),
            Token::LBrace => self.parse_block_statement(),
            Token::Void
            | Token::Char
            | Token::Short
            | Token::Int
            | Token::Long
            | Token::Float
            | Token::Double => {
                let (ty, name) = self.parse_type_declaration()?;
                self.next_token();
                match self.current_token.clone() {
                    Token::Assignment | Token::SemiColon => {
                        self.parse_variable_declaration(ty, name)
                    }
                    Token::LParen => self.parse_function_declaration(name),
                    _ => Err(format!(
                        "expected token '=' or '(' but got {:?}",
                        self.current_token
                    )),
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_block_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // skip '{'
        let mut statements = Vec::new();
        while self.current_token != Token::RBrace {
            statements.push(self.parse_statement()?);
            self.next_token();
        }
        Ok(Statement::Block(statements))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peeked_token == Token::SemiColon || self.peeked_token == Token::RParen {
            self.next_token();
        } else {
            return Err(format!(
                "expected token ';' or ')' but got {:?}",
                self.peeked_token
            ));
        }

        Ok(Statement::Expression(expr))
    }
}

#[cfg(test)]
mod test {
    use ast::{
        BinaryOperator, CallExpression, Expression, TypeEnum, UnaryExpression, UnaryOperator,
    };

    use super::*;

    #[test]
    fn test_parse() {
        let cases = vec![
            (
                String::from("5;1+2*3;"),
                Program::new(vec![
                    Statement::Expression(Expression::Integer(5)),
                    Statement::Expression(Expression::Binary {
                        lhs: Box::new(Expression::Integer(1)),
                        op: BinaryOperator::Plus,
                        rhs: Box::new(Expression::Binary {
                            lhs: Box::new(Expression::Integer(2)),
                            op: BinaryOperator::Asterisk,
                            rhs: Box::new(Expression::Integer(3)),
                        }),
                    }),
                ]),
            ),
            (
                String::from("bar(1, 2); return 0;"),
                Program::new(vec![
                    Statement::Expression(Expression::Call(CallExpression::new(
                        String::from("bar"),
                        vec![Expression::Integer(1), Expression::Integer(2)],
                    ))),
                    Statement::Return(Expression::Integer(0)),
                ]),
            ),
            (
                String::from(
                    r#"
                        int foo(int i) {
                            return i;
                        }
                        int main() {
                            int a = foo(10);
                            return 10;
                        }"#,
                ),
                Program::new(vec![
                    Statement::FunctionDefinition {
                        name: String::from("foo"),
                        arguments: vec![Expression::LocalVariable {
                            name: String::from("i"),
                            offset: 8,
                            type_: Type::Primitive(TypeEnum::Int),
                        }],
                        body: vec![Statement::Return(Expression::LocalVariable {
                            name: String::from("i"),
                            offset: 8,
                            type_: Type::Primitive(TypeEnum::Int),
                        })],
                    },
                    Statement::FunctionDefinition {
                        name: String::from("main"),
                        arguments: vec![],
                        body: vec![
                            Statement::InitDeclaration {
                                name: String::from("a"),
                                offset: 16,
                                type_: Type::Primitive(TypeEnum::Int),
                                init: Some(Expression::Call(CallExpression::new(
                                    String::from("foo"),
                                    vec![Expression::Integer(10)],
                                ))),
                            },
                            Statement::Return(Expression::Integer(10)),
                        ],
                    },
                ]),
            ),
            (
                String::from(
                    r#"
                        int one(int *x) {
                            *x = 1;
                            return 0;
                        }
                        int main() {
                            int x = 0;
                            one(&x);
                            return x;
                        }
                    "#,
                ),
                Program::new(vec![
                    Statement::FunctionDefinition {
                        name: String::from("one"),
                        arguments: vec![Expression::LocalVariable {
                            name: String::from("x"),
                            offset: 8,
                            type_: Type::Pointer(Box::new(Type::Primitive(TypeEnum::Int))),
                        }],
                        body: vec![
                            Statement::Expression(Expression::Binary {
                                lhs: Box::new(Expression::Unary(UnaryExpression::new(
                                    Expression::LocalVariable {
                                        name: String::from("x"),
                                        offset: 8,
                                        type_: Type::Pointer(Box::new(Type::Primitive(
                                            TypeEnum::Int,
                                        ))),
                                    },
                                    UnaryOperator::Dereference,
                                ))),
                                op: BinaryOperator::Assignment,
                                rhs: Box::new(Expression::Integer(1)),
                            }),
                            Statement::Return(Expression::Integer(0)),
                        ],
                    },
                    Statement::FunctionDefinition {
                        name: String::from("main"),
                        arguments: vec![],
                        body: vec![
                            Statement::InitDeclaration {
                                name: String::from("x"),
                                offset: 16,
                                type_: Type::Primitive(TypeEnum::Int),
                                init: Some(Expression::Integer(0)),
                            },
                            Statement::Expression(Expression::Call(CallExpression::new(
                                String::from("one"),
                                vec![Expression::Unary(UnaryExpression::new(
                                    Expression::LocalVariable {
                                        name: String::from("x"),
                                        offset: 8,
                                        type_: Type::Pointer(Box::new(Type::Primitive(
                                            TypeEnum::Int,
                                        ))),
                                    },
                                    UnaryOperator::Reference,
                                ))],
                            ))),
                            Statement::Return(Expression::LocalVariable {
                                name: String::from("x"),
                                offset: 8,
                                type_: Type::Pointer(Box::new(Type::Primitive(TypeEnum::Int))),
                            }),
                        ],
                    },
                ]),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap(), expected);
        }
    }

    #[test]
    fn test_parse_block_statement() {
        let cases = vec![
            (String::from("{}"), Statement::Block(vec![])),
            (
                String::from("{ return 0; }"),
                Statement::Block(vec![Statement::Return(Expression::Integer(0))]),
            ),
            (
                String::from("{ int i = 0; i = i + 1; return 0; }"),
                Statement::Block(vec![
                    Statement::InitDeclaration {
                        name: String::from("i"),
                        offset: 8,
                        type_: Type::Primitive(TypeEnum::Int),
                        init: Some(Expression::Integer(0)),
                    },
                    Statement::Expression(Expression::Binary {
                        lhs: Box::new(Expression::LocalVariable {
                            name: String::from("i"),
                            offset: 8,
                            type_: Type::Primitive(TypeEnum::Int),
                        }),
                        op: BinaryOperator::Assignment,
                        rhs: Box::new(Expression::Binary {
                            lhs: Box::new(Expression::LocalVariable {
                                name: String::from("i"),
                                offset: 8,
                                type_: Type::Primitive(TypeEnum::Int),
                            }),
                            op: BinaryOperator::Plus,
                            rhs: Box::new(Expression::Integer(1)),
                        }),
                    }),
                    Statement::Return(Expression::Integer(0)),
                ]),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse_statement().unwrap(), expected);
        }
    }
}
