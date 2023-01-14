use crate::{
    ast::{
        BinaryExpression, BinaryOperator, Expression, Program, Statement, UnaryExpression,
        UnaryOperator,
    },
    lex::{Lexer, Token},
};

// entry
pub(crate) fn parse(input: String) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse().unwrap()
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
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next();
        let peeked_token = lexer.next();
        Self {
            lexer,
            current_token,
            peeked_token,
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
            // TODO: other statements
            _ => {
                let expr = self.parse_expression_statement();
                self.next_token(); // skip ';'
                expr
            }
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        Ok(Statement::Expression(expr))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let mut expr = match self.current_token.clone() {
            Token::Integer(n) => Expression::Integer(n),
            Token::LParen => self.parse_grouped_expression()?,
            Token::Minus => self.parse_unary_expression()?,
            Token::Identifier(s) => Expression::LocalVariable(s),
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
    fn test_parse_local_var() {
        let cases = vec![
            (
                String::from("a"),
                Expression::LocalVariable(String::from("a")),
            ),
            (
                String::from("a + 2"),
                Expression::Binary(BinaryExpression::new(
                    Expression::LocalVariable(String::from("a")),
                    BinaryOperator::Plus,
                    Expression::Integer(2),
                )),
            ),
            (
                String::from("a = b"),
                Expression::Binary(BinaryExpression::new(
                    Expression::LocalVariable(String::from("a")),
                    BinaryOperator::Assignment,
                    Expression::LocalVariable(String::from("b")),
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
    fn test_parse() {
        let cases = vec![(
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
        )];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap(), expected);
        }
    }
}
