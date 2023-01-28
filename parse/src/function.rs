use ast::Statement;
use lex::tokens::Token;

use crate::{Parser, Precedence};

impl Parser {
    pub(crate) fn parse_function_declaration(&mut self, name: String) -> Result<Statement, String> {
        let mut arguments = Vec::new();
        while self.peeked_token != Token::RParen {
            self.next_token();
            let (type_, name) = self.parse_type_declaration()?;

            if self.peeked_token == Token::Comma {
                self.next_token();
            }

            arguments.push((type_, name));
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

        let arguments = arguments
            .iter()
            .map(|(t, name)| self.new_local_var(t.clone(), name.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        let body = match self.parse_block_statement()? {
            Statement::Block(body) => body,
            _ => unreachable!(),
        };

        Ok(Statement::FunctionDefinition {
            name,
            arguments,
            body,
        })
    }

    pub(crate) fn parse_return_statement(&mut self) -> Result<Statement, String> {
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
}

#[cfg(test)]
mod test {
    use ast::{BinaryExpression, BinaryOperator, Expression, Type, TypeEnum};
    use lex::Lexer;

    use super::*;

    #[test]
    fn test_parse_function_definition() {
        let cases = vec![
            (
                String::from("int foo() { return 0; }"),
                Statement::FunctionDefinition {
                    name: String::from("foo"),
                    arguments: vec![],
                    body: vec![Statement::Return(Expression::Integer(0))],
                },
            ),
            (
                String::from("int foo(int a, int b) { return 0; }"),
                Statement::FunctionDefinition {
                    name: String::from("foo"),
                    arguments: vec![
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
                    body: vec![Statement::Return(Expression::Integer(0))],
                },
            ),
        ];
        for (input, expected) in cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            assert_eq!(parser.parse_statement().unwrap(), expected);
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
}
