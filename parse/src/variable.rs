use ast::{Expression, InitDeclaration, Statement, Type, TypeEnum};
use lex::tokens::Token;

use crate::{LVar, Parser, Precedence};

impl Parser {
    pub(super) fn parse_variable_declaration(
        &mut self,
        type_: Type,
        name: String,
    ) -> Result<Statement, String> {
        let offset = match self.new_local_var(type_.clone(), name.clone())? {
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
            name, offset, type_, init_expr,
        )))
    }
    pub(super) fn new_local_var(
        &mut self,
        type_: Type,
        name: String,
    ) -> Result<Expression, String> {
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
    pub(super) fn find_local_var(&self, name: &str) -> Option<&LVar> {
        self.locals.iter().find(|s| s.name == name)
    }
    pub(super) fn parse_type_declaration(&mut self) -> Result<(Type, String), String> {
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
}

#[cfg(test)]
mod test {
    use lex::Lexer;

    use super::*;

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
}
