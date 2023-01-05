#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Token {
    Plus,
    Minus,
    Slash,
    Asterisk,
    LParen,
    RParen,
    Integer(i32),
    Eof,
}

pub(crate) struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub(crate) fn next(&mut self) -> Token {
        self.skip_whitespace();
        match self.ch {
            '+' => {
                self.read_char();
                Token::Plus
            }
            '-' => {
                self.read_char();
                Token::Minus
            }
            '*' => {
                self.read_char();
                Token::Asterisk
            }
            '/' => {
                self.read_char();
                Token::Slash
            }
            '(' => {
                self.read_char();
                Token::LParen
            }
            ')' => {
                self.read_char();
                Token::RParen
            }
            '\0' => {
                self.read_char();
                Token::Eof
            }
            _ => {
                if self.ch.is_numeric() {
                    let num = self.read_number();
                    Token::Integer(num)
                } else {
                    self.report_error("invalid token");
                    panic!();
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> i32 {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[position..self.position].parse().unwrap()
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn report_error(&self, message: &str) {
        let mut error = String::new();
        error.push_str("\x1b[31merror\x1b[0m: ");
        error.push_str(message);
        error.push_str("\n");
        error.push_str(&self.input);
        error.push_str("\n");
        error.push_str(&" ".repeat(self.position));
        error.push_str("\x1b[33m^\x1b[0m\n");
        println!("{}", error);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        {
            let input = "1+-/*()";
            let mut lexer = Lexer::new(input);
            assert_eq!(lexer.next(), Token::Integer(1));
            assert_eq!(lexer.next(), Token::Plus);
            assert_eq!(lexer.next(), Token::Minus);
            assert_eq!(lexer.next(), Token::Slash);
            assert_eq!(lexer.next(), Token::Asterisk);
            assert_eq!(lexer.next(), Token::LParen);
            assert_eq!(lexer.next(), Token::RParen);
            assert_eq!(lexer.next(), Token::Eof);
        }
        {
            let input = "1 + 2";
            let mut lexer = Lexer::new(input);

            assert_eq!(lexer.next(), Token::Integer(1));
            assert_eq!(lexer.next(), Token::Plus);
            assert_eq!(lexer.next(), Token::Integer(2));
            assert_eq!(lexer.next(), Token::Eof);
        }

        {
            let input = "5+20-4";
            let mut lexer = Lexer::new(input);

            assert_eq!(lexer.next(), Token::Integer(5));
            assert_eq!(lexer.next(), Token::Plus);
            assert_eq!(lexer.next(), Token::Integer(20));
            assert_eq!(lexer.next(), Token::Minus);
            assert_eq!(lexer.next(), Token::Integer(4));
            assert_eq!(lexer.next(), Token::Eof);
        }
    }
}
