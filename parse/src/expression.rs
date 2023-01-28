use ast::{
    BinaryExpression, BinaryOperator, CallExpression, Expression, UnaryExpression, UnaryOperator,
};
use lex::tokens::Token;

use crate::{LVar, Parser, Precedence};

impl Parser {
    pub(super) fn parse_expression(
        &mut self,
        precedence: Precedence,
    ) -> Result<Expression, String> {
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
            Token::LBrace => self.parse_array_expression()?,

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
                Token::LBracket => {
                    self.next_token();
                    self.perse_index_expression(expr)?
                }
                _ => panic!(""), // TODO:
            }
        }

        Ok(expr)
    }

    pub(super) fn parse_unary_expression(&mut self) -> Result<Expression, String> {
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

    pub(super) fn parse_binary_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, String> {
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

    pub(super) fn parse_grouped_expression(&mut self) -> Result<Expression, String> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token != Token::RParen {
            return Err(format!("Expected ')', but got {:?}", self.peeked_token));
        }
        self.next_token();
        Ok(expr)
    }

    pub(super) fn parse_identifier_expression(
        &mut self,
        name: String,
    ) -> Result<Expression, String> {
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

    pub(super) fn parse_call_expression(
        &mut self,
        callee_name: String,
    ) -> Result<Expression, String> {
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

    pub(super) fn perse_index_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, String> {
        self.next_token(); // skip '['
        let index = self.parse_expression(Precedence::Lowest)?;
        if self.peeked_token != Token::RBracket {
            return Err(format!("Expected ']', but got {:?}", self.peeked_token));
        }
        self.next_token(); // skip ']'

        Ok(Expression::Index {
            expr: Box::new(left),
            index: Box::new(index),
        })
    }

    // TODO: valid only initial declaration
    pub(crate) fn parse_array_expression(&mut self) -> Result<Expression, String> {
        let mut elements = vec![];

        while self.peeked_token != Token::RBrace {
            self.next_token();
            let expr = self.parse_expression(Precedence::Lowest)?;
            elements.push(expr);

            if self.peeked_token == Token::Comma {
                self.next_token(); // skip ','
            }
        }

        self.next_token();
        self.next_token(); // skip '}'

        Ok(Expression::Array { elements })
    }
}

#[cfg(test)]
mod test {
    use ast::{InitDeclaration, Statement, Type, TypeEnum};
    use lex::Lexer;

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
    fn parse_index_expression() {
        let cases = vec![(
            String::from("int foo[3]; foo[1];"),
            vec![
                Statement::InitDeclaration(InitDeclaration::new(
                    String::from("foo"),
                    24,
                    Type::Array {
                        type_: Box::new(Type::Primitive(TypeEnum::Int)),
                        size: 3,
                    },
                    None,
                )),
                Statement::Expression(Expression::Index {
                    expr: Box::new(Expression::LocalVariable {
                        name: String::from("foo"),
                        offset: 24,
                        type_: Type::Array {
                            type_: Box::new(Type::Primitive(TypeEnum::Int)),
                            size: 3,
                        },
                    }),
                    index: Box::new(Expression::Integer(1)),
                }),
            ],
        )];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap().statements, expected);
        }
    }

    #[test]
    fn test_parse_array_expression() {
        let cases = vec![(
            String::from("int array[3] = { 1, 2, 3 };"),
            Statement::InitDeclaration(InitDeclaration::new(
                String::from("array"),
                24,
                Type::Array {
                    type_: Box::new(Type::Primitive(TypeEnum::Int)),
                    size: 3,
                },
                Some(Expression::Array {
                    elements: vec![
                        Expression::Integer(1),
                        Expression::Integer(2),
                        Expression::Integer(3),
                    ],
                }),
            )),
        )];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap().statements, vec![expected]);
        }
    }
}
