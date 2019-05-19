pub mod token;

use std::iter::Peekable;
use std::str::Chars;
use token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer{
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('=') => Token::Assign,
            Some(';') => Token::Semicolon,
            Some('(') => Token::Lparen,
            Some(')') => Token::Rparen,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some(ch) => {
                if is_letter(ch) {
                    let ident = self.read_identifier(ch);
                    token::lookup_ident(ident)
                } else if ch.is_digit(10) {
                    Token::Int(self.read_number(ch))
                } else {
                    Token::Illegal(ch)
                }
            },
            None => Token::EOF
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peak_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn read_identifier(&mut self, ch: char) -> String {
        let mut identifier = String::new();
        identifier.push(ch);

        while let Some(&ch) = self.peak_char() {
            if is_letter(ch) {
                identifier.push(self.read_char().unwrap());
            } else {
                break;
            }
        }

        identifier
    }

    fn read_number(&mut self, ch: char) -> i64 {
        let mut s = String::new();
        s.push(ch);

        while let Some(&ch) = self.peak_char() {
            if ch.is_digit(10) {
                s.push(self.read_char().unwrap());
            } else {
                break;
            }
        }

        s.parse().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peak_char() {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_' || ch == '?' || ch == '!'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
}

let result = add(five, ten);"#;

        let tests = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::Lparen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Rparen,
            Token::Lbrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::Lparen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::Rparen,
            Token::Semicolon,
            Token::EOF
        ];

        let mut l = Lexer::new(input);

        for tok in tests {
            assert_eq!(l.next_token(), tok)
        }
    }
}