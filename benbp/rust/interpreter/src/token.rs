// use std::fmt;

// impl fmt::Display for Token {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self.literal {
//             Some(l) => { write!(f, "{}", self.literal) }
//             None => { write!(f, "None") }
//         }
// }
//
// impl fmt::Display for Literal {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: i32, literal: Option<Literal>) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }
}

pub enum Literal {
    NumberLiteral(f64),
    StringLiteral(String),
    IdentifierLiteral(String),
}

pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}
