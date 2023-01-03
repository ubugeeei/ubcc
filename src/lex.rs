#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Plus,
    Minus,
    Integer(i32),
    Eof,
    Illegal(char),
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
            '\0' => {
                self.read_char();
                Token::Eof
            }
            _ => {
                if self.ch.is_numeric() {
                    let num = self.read_number();
                    Token::Integer(num)
                } else {
                    Token::Illegal(self.ch)
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
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
