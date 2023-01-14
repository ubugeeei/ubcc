#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Token {
    Plus,
    Minus,
    Slash,
    Asterisk,
    LParen,
    RParen,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,
    Not,
    Assignment,
    Integer(i32),
    Identifier(String),
    SemiColon,
    Eof,
}

pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub(crate) fn new(input: String) -> Self {
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
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::NotEq
                } else {
                    self.read_char();
                    Token::Not
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::Eq
                } else {
                    self.read_char();
                    Token::Assignment
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::LtEq
                } else {
                    self.read_char();
                    Token::Lt
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::GtEq
                } else {
                    self.read_char();
                    Token::Gt
                }
            }
            ';' => {
                self.read_char();
                Token::SemiColon
            }
            _ => {
                if self.ch.is_numeric() {
                    let num = self.read_number();
                    Token::Integer(num)
                } else if self.ch >= 'a' && self.ch <= 'z' {
                    // tokenize identifier when ch is a to z
                    let c = self.ch;
                    self.read_char();
                    Token::Identifier(c.to_string())
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

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
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
            let input = String::from("1 + - / * ( ) = ! == != < > <= >= ; a b z");
            let mut lexer = Lexer::new(input);
            assert_eq!(lexer.next(), Token::Integer(1));
            assert_eq!(lexer.next(), Token::Plus);
            assert_eq!(lexer.next(), Token::Minus);
            assert_eq!(lexer.next(), Token::Slash);
            assert_eq!(lexer.next(), Token::Asterisk);
            assert_eq!(lexer.next(), Token::LParen);
            assert_eq!(lexer.next(), Token::RParen);
            assert_eq!(lexer.next(), Token::Assignment);
            assert_eq!(lexer.next(), Token::Not);
            assert_eq!(lexer.next(), Token::Eq);
            assert_eq!(lexer.next(), Token::NotEq);
            assert_eq!(lexer.next(), Token::Lt);
            assert_eq!(lexer.next(), Token::Gt);
            assert_eq!(lexer.next(), Token::LtEq);
            assert_eq!(lexer.next(), Token::GtEq);
            assert_eq!(lexer.next(), Token::SemiColon);
            assert_eq!(lexer.next(), Token::Identifier(String::from("a")));
            assert_eq!(lexer.next(), Token::Identifier(String::from("b")));
            assert_eq!(lexer.next(), Token::Identifier(String::from("z")));
            assert_eq!(lexer.next(), Token::Eof);
        }
        {
            let input = String::from("1 + 2");
            let mut lexer = Lexer::new(input);

            assert_eq!(lexer.next(), Token::Integer(1));
            assert_eq!(lexer.next(), Token::Plus);
            assert_eq!(lexer.next(), Token::Integer(2));
            assert_eq!(lexer.next(), Token::Eof);
        }

        {
            let input = String::from("5+20-4");
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
