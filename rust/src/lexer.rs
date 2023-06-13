use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer<'_> {
        let input = input.as_bytes();
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let tok = match self.ch? {
            b'=' => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Some(Token::Eq)
                } else {
                    Some(Token::Assign)
                }
            }
            b'+' => Some(Token::Plus),
            b'-' => Some(Token::Minus),
            b'!' => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Some(Token::NotEq)
                } else {
                    Some(Token::Bang)
                }
            }
            b'/' => Some(Token::Slash),
            b'*' => Some(Token::Asterisk),
            b'<' => Some(Token::LT),
            b'>' => Some(Token::GT),
            b';' => Some(Token::Semicolon),
            b',' => Some(Token::Comma),
            b'(' => Some(Token::LParen),
            b')' => Some(Token::RParen),
            b'{' => Some(Token::LBrace),
            b'}' => Some(Token::RBrace),
            c if is_letter(c) => {
                let literal = self.read_identifier();
                return Some(Token::lookup_ident(literal));
            }
            c if is_digit(c) => {
                return Some(Token::Int(self.read_number().to_owned()));
            }
            _ => Some(Token::Illegal),
        };
        self.read_char();
        tok
    }

    /// read_char reads the next character to ch and advances the position in
    /// the input.
    fn read_char(&mut self) {
        self.ch = self.input.get(self.read_position).copied();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> Option<u8> {
        self.input.get(self.read_position).copied()
    }

    fn read_identifier(&mut self) -> &str {
        self.read_while(is_letter)
    }

    fn read_number(&mut self) -> &str {
        self.read_while(is_digit)
    }

    fn read_while<P: Fn(u8) -> bool>(&mut self, p: P) -> &str {
        let pos = self.position;
        while let Some(c) = self.ch {
            if !p(c) {
                break;
            }
            self.read_char();
        }
        std::str::from_utf8(&self.input[pos..self.position]).unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.map_or(false, |c| c.is_ascii_whitespace()) {
            self.read_char();
        }
    }
}

fn is_letter(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Assign));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::LParen));
        assert_eq!(lexer.next_token(), Some(Token::RParen));
        assert_eq!(lexer.next_token(), Some(Token::LBrace));
        assert_eq!(lexer.next_token(), Some(Token::RBrace));
        assert_eq!(lexer.next_token(), Some(Token::Comma));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
    }

    #[test]
    fn test_next_token_program() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;";

        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Ident("five".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Assign));
        assert_eq!(lexer.next_token(), Some(Token::Int("5".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Ident("ten".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Assign));
        assert_eq!(lexer.next_token(), Some(Token::Int("10".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Ident("add".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Assign));
        assert_eq!(lexer.next_token(), Some(Token::Function));
        assert_eq!(lexer.next_token(), Some(Token::LParen));
        assert_eq!(lexer.next_token(), Some(Token::Ident("x".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Comma));
        assert_eq!(lexer.next_token(), Some(Token::Ident("y".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::RParen));
        assert_eq!(lexer.next_token(), Some(Token::LBrace));
        assert_eq!(lexer.next_token(), Some(Token::Ident("x".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Ident("y".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::RBrace));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::Let));
        assert_eq!(lexer.next_token(), Some(Token::Ident("result".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Assign));
        assert_eq!(lexer.next_token(), Some(Token::Ident("add".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::LParen));
        assert_eq!(lexer.next_token(), Some(Token::Ident("five".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Comma));
        assert_eq!(lexer.next_token(), Some(Token::Ident("ten".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::RParen));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::Bang));
        assert_eq!(lexer.next_token(), Some(Token::Minus));
        assert_eq!(lexer.next_token(), Some(Token::Slash));
        assert_eq!(lexer.next_token(), Some(Token::Asterisk));
        assert_eq!(lexer.next_token(), Some(Token::Int("5".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::Int("5".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::LT));
        assert_eq!(lexer.next_token(), Some(Token::Int("10".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::GT));
        assert_eq!(lexer.next_token(), Some(Token::Int("5".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::If));
        assert_eq!(lexer.next_token(), Some(Token::LParen));
        assert_eq!(lexer.next_token(), Some(Token::Int("5".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::LT));
        assert_eq!(lexer.next_token(), Some(Token::Int("10".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::RParen));
        assert_eq!(lexer.next_token(), Some(Token::LBrace));
        assert_eq!(lexer.next_token(), Some(Token::Return));
        assert_eq!(lexer.next_token(), Some(Token::True));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::RBrace));
        assert_eq!(lexer.next_token(), Some(Token::Else));
        assert_eq!(lexer.next_token(), Some(Token::LBrace));
        assert_eq!(lexer.next_token(), Some(Token::Return));
        assert_eq!(lexer.next_token(), Some(Token::False));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::RBrace));
        assert_eq!(lexer.next_token(), Some(Token::Int("10".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Eq));
        assert_eq!(lexer.next_token(), Some(Token::Int("10".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), Some(Token::Int("10".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::NotEq));
        assert_eq!(lexer.next_token(), Some(Token::Int("9".to_owned())));
        assert_eq!(lexer.next_token(), Some(Token::Semicolon));
        assert_eq!(lexer.next_token(), None);
    }
}
