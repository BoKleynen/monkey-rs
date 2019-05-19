use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("let", Token::Let);
        m.insert("fn", Token::Function);
        m
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    Illegal(char),
    EOF,

    Ident(String),
    Int(i64),

    Assign,
    Plus,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
}

pub fn lookup_ident(ident: String) -> Token {
    match KEYWORDS.get(ident.as_str()) {
        Some(token) => token.clone(),
        None => Token::Ident(ident),
    }
}
