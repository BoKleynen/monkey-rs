pub enum Token {
    Illegal,
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