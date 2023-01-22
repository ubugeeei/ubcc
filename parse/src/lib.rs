use ast::{
    BinaryExpression, BinaryOperator, CallExpression, Expression, FunctionDefinition,
    InitDeclaration, Program, Statement, Type, TypeEnum, UnaryExpression, UnaryOperator,
};
use lex::{tokens::Token, Lexer};

mod branch;
mod loop_;

// entry
pub fn parse(input: String) -> Result<Program, String> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
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
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peeked_token: Token,
    locals: Vec<LVar>,
}

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

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // skip 'return'
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peeked_token == Token::SemiColon {
            self.next_token();
        } else {
            return Err(format!(
                "expected token ';' but got {:?}",
                self.peeked_token
            ));
        }

        Ok(Statement::Return(expr))
    }

    fn parse_variable_declaration(
        &mut self,
        type_: Type,
        name: String,
    ) -> Result<Statement, String> {
        let offset = match self.new_local_var(type_.clone(), name.clone())? {
            // TODO: size
            Expression::LocalVariable { offset, .. } => offset,
            _ => unreachable!(),
        };

        let init_expr = match self.current_token {
            Token::SemiColon => None,
            Token::Assignment => {
                self.next_token();
                let e = Some(self.parse_expression(Precedence::Lowest)?);
                self.next_token();
                e
            }
            _ => {
                return Err(format!(
                    "expected token ';' but got {:?}",
                    self.current_token
                ))
            }
        };

        Ok(Statement::InitDeclaration(InitDeclaration::new(
            name, offset, type_, // TODO: other types
            init_expr,
        )))
    }

    fn parse_function_declaration(&mut self, name: String) -> Result<Statement, String> {
        let mut params = Vec::new();
        while self.peeked_token != Token::RParen {
            self.next_token();
            let (type_, name) = self.parse_type_declaration()?;

            if self.peeked_token == Token::Comma {
                self.next_token();
            }

            params.push((type_, name));
        }

        if self.peeked_token == Token::RParen {
            self.next_token();
            self.next_token(); // skip ')'
        } else {
            return Err(format!(
                "expected token ')' but got {:?}",
                self.peeked_token
            ));
        }

        let params = params
            .iter()
            .map(|(t, name)| self.new_local_var(t.clone(), name.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        let body = match self.parse_block_statement()? {
            Statement::Block(body) => body,
            _ => unreachable!(),
        };

        Ok(Statement::FunctionDefinition(FunctionDefinition::new(
            name, params, body,
        )))
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

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let mut expr = match self.current_token.clone() {
            Token::Integer(n) => Expression::Integer(n),
            Token::LParen => self.parse_grouped_expression()?,
            Token::Minus | Token::Asterisk | Token::Ampersand => self.parse_unary_expression()?,
            Token::Identifier(name) => match self.peeked_token {
                Token::LParen => {
                    self.next_token(); // skip identifier
                    self.parse_call_expression(name)?
                }
                _ => self.parse_identifier_expression(name)?,
            },

            _ => return Err(format!("Invalid token: {:?}", self.current_token)),
        };

        while self.peeked_token != Token::Eof && precedence < self.peek_precedence() {
            expr = match self.peeked_token {
                Token::Assignment
                | Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::Slash
                | Token::Eq
                | Token::NotEq
                | Token::Lt
                | Token::Gt
                | Token::LtEq
                | Token::GtEq => {
                    self.next_token();
                    self.parse_binary_expression(expr)?
                }
                _ => panic!(""), // TODO:
            }
        }

        Ok(expr)
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, String> {
        match self.current_token {
            Token::Minus => {
                self.next_token();
                let expr = self.parse_expression(Precedence::Product)?;
                Ok(Expression::Unary(UnaryExpression::new(
                    expr,
                    UnaryOperator::Minus,
                )))
            }
            Token::Asterisk => {
                self.next_token();
                let expr = self.parse_expression(Precedence::Product)?;
                Ok(Expression::Unary(UnaryExpression::new(
                    expr,
                    UnaryOperator::Dereference,
                )))
            }
            Token::Ampersand => {
                self.next_token();
                let expr = self.parse_expression(Precedence::Product)?;
                Ok(Expression::Unary(UnaryExpression::new(
                    expr,
                    UnaryOperator::Reference,
                )))
            }
            _ => unreachable!(),
        }
    }

    fn parse_binary_expression(&mut self, left: Expression) -> Result<Expression, String> {
        let (op, swap) = match self.current_token {
            Token::Assignment => (BinaryOperator::Assignment, false),
            Token::Plus => (BinaryOperator::Plus, false),
            Token::Minus => (BinaryOperator::Minus, false),
            Token::Asterisk => (BinaryOperator::Asterisk, false),
            Token::Slash => (BinaryOperator::Slash, false),
            Token::Lt => (BinaryOperator::Lt, false),
            Token::Gt => (BinaryOperator::Lt, true),
            Token::LtEq => (BinaryOperator::LtEq, false),
            Token::GtEq => (BinaryOperator::LtEq, true),
            Token::Eq => (BinaryOperator::Eq, false),
            Token::NotEq => (BinaryOperator::NotEq, false),
            _ => {
                return Err(format!(
                    "Expected binary operator, but got {:?}",
                    self.current_token
                ))
            }
        };
        let precedence = self.get_precedence(self.current_token.clone());
        self.next_token();
        let right = self.parse_expression(precedence)?;

        // when swap is true, swap left and right
        if swap {
            Ok(Expression::Binary(BinaryExpression::new(right, op, left)))
        } else {
            Ok(Expression::Binary(BinaryExpression::new(left, op, right)))
        }
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, String> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token != Token::RParen {
            return Err(format!("Expected ')', but got {:?}", self.peeked_token));
        }
        self.next_token();
        Ok(expr)
    }

    fn parse_identifier_expression(&mut self, name: String) -> Result<Expression, String> {
        let offset = self.find_local_var(&name);
        match offset {
            Some(LVar { offset, type_, .. }) => Ok(Expression::LocalVariable {
                name,
                offset: *offset,
                type_: type_.clone(),
            }),
            None => Err(format!("Undefined variable: {}", name)),
        }
    }

    fn parse_call_expression(&mut self, callee_name: String) -> Result<Expression, String> {
        let mut arguments = vec![];

        while self.peeked_token != Token::RParen {
            self.next_token();
            let expr = self.parse_expression(Precedence::Lowest)?;
            arguments.push(expr);
            if self.peeked_token == Token::Comma {
                self.next_token();
            }
        }

        self.next_token(); // skip ')'

        Ok(Expression::Call(CallExpression::new(
            callee_name,
            arguments,
        )))
    }

    fn new_local_var(&mut self, type_: Type, name: String) -> Result<Expression, String> {
        let offset = self.locals.last().map(|l| l.offset).unwrap_or(0) + type_.size();
        let v = LVar {
            name: name.clone(),
            offset,
            type_: type_.clone(),
        };
        self.locals.push(v);
        Ok(Expression::LocalVariable {
            name,
            offset,
            type_,
        })
    }

    fn parse_type_declaration(&mut self) -> Result<(Type, String), String> {
        let base = match self.current_token {
            Token::Void => Type::Primitive(TypeEnum::Void),
            Token::Char => Type::Primitive(TypeEnum::Char),
            Token::Short => Type::Primitive(TypeEnum::Short),
            Token::Int => Type::Primitive(TypeEnum::Int),
            Token::Long => Type::Primitive(TypeEnum::Long),
            Token::Float => Type::Primitive(TypeEnum::Float),
            Token::Double => Type::Primitive(TypeEnum::Double),
            _ => return Err(format!("Expected type, but got {:?}", self.current_token)),
        };
        self.next_token();

        let mut p_count = 0;
        while self.current_token == Token::Asterisk {
            p_count += 1;
            self.next_token();
        }

        let name = match self.current_token.clone() {
            Token::Identifier(name) => name,
            _ => {
                return Err(format!(
                    "Expected identifier, but got {:?}",
                    self.current_token
                ))
            }
        };

        let mut t = base;
        // parse array
        while self.peeked_token == Token::LBracket {
            self.next_token(); // skip '['

            let Token::Integer(size) = self.peeked_token else {
                return Err(format!("Expected integer, but got {:?}", self.peeked_token));
            };
            self.next_token();

            self.next_token(); // skip ']'

            t = Type::Array {
                type_: Box::new(t),
                size,
            };
        }
        for _ in 0..p_count {
            t = Type::Pointer(Box::new(t));
        }

        Ok((t, name))
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
            _ => Precedence::Lowest,
        }
    }

    fn find_local_var(&self, name: &str) -> Option<&LVar> {
        self.locals.iter().find(|s| s.name == name)
    }

    fn next_token(&mut self) {
        self.current_token = self.peeked_token.clone();
        self.peeked_token = self.lexer.next();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_integer() {
        let cases = vec![
            (String::from("5"), Expression::Integer(5)),
            (String::from("10"), Expression::Integer(10)),
            (
                String::from("-10"),
                Expression::Unary(UnaryExpression::new(
                    Expression::Integer(10),
                    UnaryOperator::Minus,
                )),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(
                parser.parse_expression(Precedence::Lowest).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn test_parse_unary_expression() {
        let cases = vec![
            (
                String::from("&5"),
                Expression::Unary(UnaryExpression::new(
                    Expression::Integer(5),
                    UnaryOperator::Reference,
                )),
            ),
            (
                String::from("*5"),
                Expression::Unary(UnaryExpression::new(
                    Expression::Integer(5),
                    UnaryOperator::Dereference,
                )),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(
                parser.parse_expression(Precedence::Lowest).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn test_binary_expression() {
        let case = vec![
            (
                String::from("5 + 5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Integer(5),
                    BinaryOperator::Plus,
                    Expression::Integer(5),
                )),
            ),
            (
                String::from("5 - 5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Integer(5),
                    BinaryOperator::Minus,
                    Expression::Integer(5),
                )),
            ),
            (
                String::from("5 * 5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Integer(5),
                    BinaryOperator::Asterisk,
                    Expression::Integer(5),
                )),
            ),
            (
                String::from("5 / 5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Integer(5),
                    BinaryOperator::Slash,
                    Expression::Integer(5),
                )),
            ),
            // include unary
            (
                String::from("-5 + 5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Unary(UnaryExpression::new(
                        Expression::Integer(5),
                        UnaryOperator::Minus,
                    )),
                    BinaryOperator::Plus,
                    Expression::Integer(5),
                )),
            ),
            (
                String::from("5 + -5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Integer(5),
                    BinaryOperator::Plus,
                    Expression::Unary(UnaryExpression::new(
                        Expression::Integer(5),
                        UnaryOperator::Minus,
                    )),
                )),
            ),
        ];

        for (input, expected) in case {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(
                parser.parse_expression(Precedence::Lowest).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn test_binary_expression_with_precedence() {
        let cases = vec![
            (
                String::from("5 + 5 * 5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Integer(5),
                    BinaryOperator::Plus,
                    Expression::Binary(BinaryExpression::new(
                        Expression::Integer(5),
                        BinaryOperator::Asterisk,
                        Expression::Integer(5),
                    )),
                )),
            ),
            (
                String::from("1 * 2 + 3 * 4"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Binary(BinaryExpression::new(
                        Expression::Integer(1),
                        BinaryOperator::Asterisk,
                        Expression::Integer(2),
                    )),
                    BinaryOperator::Plus,
                    Expression::Binary(BinaryExpression::new(
                        Expression::Integer(3),
                        BinaryOperator::Asterisk,
                        Expression::Integer(4),
                    )),
                )),
            ),
            (
                String::from("1 * 2 >= 3 * 4 == 0"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Binary(BinaryExpression::new(
                        Expression::Binary(BinaryExpression::new(
                            Expression::Integer(3),
                            BinaryOperator::Asterisk,
                            Expression::Integer(4),
                        )),
                        BinaryOperator::LtEq,
                        Expression::Binary(BinaryExpression::new(
                            Expression::Integer(1),
                            BinaryOperator::Asterisk,
                            Expression::Integer(2),
                        )),
                    )),
                    BinaryOperator::Eq,
                    Expression::Integer(0),
                )),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(
                parser.parse_expression(Precedence::Lowest).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn test_binary_expression_with_paren() {
        let cases = vec![
            (
                String::from("(5 + 5) * 5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Binary(BinaryExpression::new(
                        Expression::Integer(5),
                        BinaryOperator::Plus,
                        Expression::Integer(5),
                    )),
                    BinaryOperator::Asterisk,
                    Expression::Integer(5),
                )),
            ),
            (
                String::from("1 * (2 + 3) * 4"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Binary(BinaryExpression::new(
                        Expression::Integer(1),
                        BinaryOperator::Asterisk,
                        Expression::Binary(BinaryExpression::new(
                            Expression::Integer(2),
                            BinaryOperator::Plus,
                            Expression::Integer(3),
                        )),
                    )),
                    BinaryOperator::Asterisk,
                    Expression::Integer(4),
                )),
            ),
            (
                String::from("1 * (2 * (3 + 4)) * 5"),
                Expression::Binary(BinaryExpression::new(
                    Expression::Binary(BinaryExpression::new(
                        Expression::Integer(1),
                        BinaryOperator::Asterisk,
                        Expression::Binary(BinaryExpression::new(
                            Expression::Integer(2),
                            BinaryOperator::Asterisk,
                            Expression::Binary(BinaryExpression::new(
                                Expression::Integer(3),
                                BinaryOperator::Plus,
                                Expression::Integer(4),
                            )),
                        )),
                    )),
                    BinaryOperator::Asterisk,
                    Expression::Integer(5),
                )),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(
                parser.parse_expression(Precedence::Lowest).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn test_parse_return_statement() {
        let cases = vec![
            (
                String::from("return 5;"),
                Statement::Return(Expression::Integer(5)),
            ),
            (
                String::from("return 5 + 5;"),
                Statement::Return(Expression::Binary(BinaryExpression::new(
                    Expression::Integer(5),
                    BinaryOperator::Plus,
                    Expression::Integer(5),
                ))),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse_statement().unwrap(), expected);
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
                    Statement::InitDeclaration(InitDeclaration::new(
                        String::from("i"),
                        8,
                        Type::Primitive(TypeEnum::Int),
                        Some(Expression::Integer(0)),
                    )),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Expression::LocalVariable {
                            name: String::from("i"),
                            offset: 8,
                            type_: Type::Primitive(TypeEnum::Int),
                        },
                        BinaryOperator::Assignment,
                        Expression::Binary(BinaryExpression::new(
                            Expression::LocalVariable {
                                name: String::from("i"),
                                offset: 8,
                                type_: Type::Primitive(TypeEnum::Int),
                            },
                            BinaryOperator::Plus,
                            Expression::Integer(1),
                        )),
                    ))),
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

    #[test]
    fn parse_call_expression() {
        let cases = vec![
            (
                String::from("foo();"),
                Expression::Call(CallExpression::new(String::from("foo"), vec![])),
            ),
            (
                String::from("bar(1, 2);"),
                Expression::Call(CallExpression::new(
                    String::from("bar"),
                    vec![Expression::Integer(1), Expression::Integer(2)],
                )),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(
                parser.parse_expression(Precedence::Lowest).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn test_parse_init_declaration() {
        let cases = vec![
            (
                String::from("int a = 0;"),
                Statement::InitDeclaration(InitDeclaration::new(
                    String::from("a"),
                    8,
                    Type::Primitive(TypeEnum::Int),
                    Some(Expression::Integer(0)),
                )),
            ),
            (
                String::from("int i;"),
                Statement::InitDeclaration(InitDeclaration::new(
                    String::from("i"),
                    8,
                    Type::Primitive(TypeEnum::Int),
                    None,
                )),
            ),
            (
                String::from("int *i;"),
                Statement::InitDeclaration(InitDeclaration::new(
                    String::from("i"),
                    8,
                    Type::Pointer(Box::new(Type::Primitive(TypeEnum::Int))),
                    None,
                )),
            ),
            (
                String::from("int **i;"),
                Statement::InitDeclaration(InitDeclaration::new(
                    String::from("i"),
                    8,
                    Type::Pointer(Box::new(Type::Pointer(Box::new(Type::Primitive(
                        TypeEnum::Int,
                    ))))),
                    None,
                )),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse_statement().unwrap(), expected);
        }
    }

    #[test]
    fn test_parse_function_definition() {
        let cases = vec![
            (
                String::from("int foo() { return 0; }"),
                Statement::FunctionDefinition(FunctionDefinition::new(
                    String::from("foo"),
                    vec![],
                    vec![Statement::Return(Expression::Integer(0))],
                )),
            ),
            (
                String::from("int foo(int a, int b) { return 0; }"),
                Statement::FunctionDefinition(FunctionDefinition::new(
                    String::from("foo"),
                    vec![
                        Expression::LocalVariable {
                            name: String::from("a"),
                            offset: 8,
                            type_: Type::Primitive(TypeEnum::Int),
                        },
                        Expression::LocalVariable {
                            name: String::from("b"),
                            offset: 16,
                            type_: Type::Primitive(TypeEnum::Int),
                        },
                    ],
                    vec![Statement::Return(Expression::Integer(0))],
                )),
            ),
        ];
        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse_statement().unwrap(), expected);
        }
    }

    #[test]
    fn test_parse_array() {
        let cases = vec![
            (
                String::from("int a[10];"),
                vec![Statement::InitDeclaration(InitDeclaration::new(
                    String::from("a"),
                    80,
                    Type::Array {
                        type_: Box::new(Type::Primitive(TypeEnum::Int)),
                        size: 10,
                    },
                    None,
                ))],
            ),
            (
                String::from("int a[5][10];"),
                vec![Statement::InitDeclaration(InitDeclaration::new(
                    String::from("a"),
                    80,
                    Type::Array {
                        type_: Box::new(Type::Array {
                            type_: Box::new(Type::Primitive(TypeEnum::Int)),
                            size: 5,
                        }),
                        size: 10,
                    },
                    None,
                ))],
            ),
            (
                String::from("int *a[10];"),
                vec![Statement::InitDeclaration(InitDeclaration::new(
                    String::from("a"),
                    8,
                    Type::Pointer(Box::new(Type::Array {
                        type_: Box::new(Type::Primitive(TypeEnum::Int)),
                        size: 10,
                    })),
                    None,
                ))],
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap().statements, expected);
        }
    }

    #[test]
    fn test_parse() {
        let cases = vec![
            (
                String::from("5;1+2*3;"),
                Program::new(vec![
                    Statement::Expression(Expression::Integer(5)),
                    Statement::Expression(Expression::Binary(BinaryExpression::new(
                        Expression::Integer(1),
                        BinaryOperator::Plus,
                        Expression::Binary(BinaryExpression::new(
                            Expression::Integer(2),
                            BinaryOperator::Asterisk,
                            Expression::Integer(3),
                        )),
                    ))),
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
                    Statement::FunctionDefinition(FunctionDefinition::new(
                        String::from("foo"),
                        vec![Expression::LocalVariable {
                            name: String::from("i"),
                            offset: 8,
                            type_: Type::Primitive(TypeEnum::Int),
                        }],
                        vec![Statement::Return(Expression::LocalVariable {
                            name: String::from("i"),
                            offset: 8,
                            type_: Type::Primitive(TypeEnum::Int),
                        })],
                    )),
                    Statement::FunctionDefinition(FunctionDefinition::new(
                        String::from("main"),
                        vec![],
                        vec![
                            Statement::InitDeclaration(InitDeclaration::new(
                                String::from("a"),
                                16,
                                Type::Primitive(TypeEnum::Int),
                                Some(Expression::Call(CallExpression::new(
                                    String::from("foo"),
                                    vec![Expression::Integer(10)],
                                ))),
                            )),
                            Statement::Return(Expression::Integer(10)),
                        ],
                    )),
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
                    Statement::FunctionDefinition(FunctionDefinition::new(
                        String::from("one"),
                        vec![Expression::LocalVariable {
                            name: String::from("x"),
                            offset: 8,
                            type_: Type::Pointer(Box::new(Type::Primitive(TypeEnum::Int))),
                        }],
                        vec![
                            Statement::Expression(Expression::Binary(BinaryExpression::new(
                                Expression::Unary(UnaryExpression::new(
                                    Expression::LocalVariable {
                                        name: String::from("x"),
                                        offset: 8,
                                        type_: Type::Pointer(Box::new(Type::Primitive(
                                            TypeEnum::Int,
                                        ))),
                                    },
                                    UnaryOperator::Dereference,
                                )),
                                BinaryOperator::Assignment,
                                Expression::Integer(1),
                            ))),
                            Statement::Return(Expression::Integer(0)),
                        ],
                    )),
                    Statement::FunctionDefinition(FunctionDefinition::new(
                        String::from("main"),
                        vec![],
                        vec![
                            Statement::InitDeclaration(InitDeclaration::new(
                                String::from("x"),
                                16,
                                Type::Primitive(TypeEnum::Int),
                                Some(Expression::Integer(0)),
                            )),
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
                    )),
                ]),
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap(), expected);
        }
    }
}