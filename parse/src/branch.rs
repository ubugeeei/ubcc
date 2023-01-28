use ast::Statement;
use lex::tokens::Token;

use crate::{Parser, Precedence};

impl Parser {
    pub(super) fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.next_token(); // skip 'if'

        if self.current_token == Token::LParen {
            self.next_token();
        } else {
            return Err(format!(
                "expected token '(' but got {:?}",
                self.current_token
            ));
        }

        let condition = self.parse_expression(Precedence::Lowest)?;

        if self.peeked_token == Token::RParen {
            self.next_token(); // skip current
            self.next_token(); // skip ')'
        } else {
            return Err(format!(
                "expected token ')' but got {:?}",
                self.peeked_token
            ));
        }

        let consequence = self.parse_statement()?;

        let alternative = if self.peeked_token == Token::Else {
            self.next_token(); // skip current
            self.next_token(); // skip 'else'
            Some(self.parse_statement()?)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            consequence: Box::new(consequence),
            alternative: alternative.map(Box::new),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ast::{BinaryExpression, BinaryOperator, Expression, Statement, Type, TypeEnum};
    use lex::Lexer;

    #[test]
    fn test_parse_if_statement() {
        let cases = vec![
            (
                String::from("int a = 0; if (a == 0) return 0; "),
                vec![
                    Statement::InitDeclaration {
                        name: String::from("a"),
                        offset: 8,
                        type_: Type::Primitive(TypeEnum::Int),
                        init: Some(Expression::Integer(0)),
                    },
                    Statement::If {
                        condition: Expression::Binary(BinaryExpression::new(
                            Expression::LocalVariable {
                                name: String::from("a"),
                                offset: 8,
                                type_: Type::Primitive(TypeEnum::Int),
                            },
                            BinaryOperator::Eq,
                            Expression::Integer(0),
                        )),
                        consequence: Box::new(Statement::Return(Expression::Integer(0))),
                        alternative: None,
                    },
                ],
            ),
            (
                String::from("int a = 0; if (a == 0) return 0; else return 1;"),
                vec![
                    Statement::InitDeclaration {
                        name: String::from("a"),
                        offset: 8,
                        type_: Type::Primitive(TypeEnum::Int),
                        init: Some(Expression::Integer(0)),
                    },
                    Statement::If {
                        condition: Expression::Binary(BinaryExpression::new(
                            Expression::LocalVariable {
                                name: String::from("a"),
                                offset: 8,
                                type_: Type::Primitive(TypeEnum::Int),
                            },
                            BinaryOperator::Eq,
                            Expression::Integer(0),
                        )),
                        consequence: Box::new(Statement::Return(Expression::Integer(0))),
                        alternative: Some(Box::new(Statement::Return(Expression::Integer(1)))),
                    },
                ],
            ),
        ];

        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse().unwrap().statements, expected);
        }
    }
}
