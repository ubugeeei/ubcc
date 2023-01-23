use ast::{FunctionDefinition, Statement};
use lex::tokens::Token;

use crate::{Parser, Precedence};

impl Parser {
    pub(crate) fn parse_function_declaration(&mut self, name: String) -> Result<Statement, String> {
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
