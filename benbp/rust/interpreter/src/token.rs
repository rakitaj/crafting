#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(token_type: TokenType, line: i32, literal: Option<Literal>) -> Option<Self> {
        match get_token_lexeme(token_type.clone()) {
            Some(lexeme) => {
                Some(Token {
                    token_type,
                    lexeme,
                    line,
                    literal
                })
            }
            _ => { None }
        }
    }

    pub fn new_from_lexeme(lexeme: &str, line: i32, literal: Option<Literal>) -> Option<Self> {
        match get_token_type(lexeme) {
            Some(t) => {
                Some(Token {
                    token_type: t,
                    lexeme: lexeme.to_string(),
                    line,
                    literal
                })
            }
            _ => { None }
        }
    }

    pub fn new_as_type(token_type: TokenType, lexeme: String, line: i32, literal: Option<Literal>) -> Option<Self> {
        Some(Token {
            token_type,
            lexeme,
            line,
            literal,
        })
    }

    pub fn new_as_keyword(lexeme: String, line: i32, literal: Option<Literal>) -> Option<Self> {
        Some(Token {
            token_type: get_token_from_keyword(lexeme.clone()),
            lexeme: lexeme.clone(),
            line,
            literal,
        })
    }
}

#[derive(Debug)]
pub enum Literal {
    NumberLiteral(f64),
    StringLiteral(String),
    IdentifierLiteral(String),
}

#[derive(Debug)]
#[derive(Clone)]
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

fn get_token_lexeme(token: TokenType) -> Option<String> {
    match token {
        TokenType::LeftParen => Some("(".to_string()),
        TokenType::RightParen => Some(")".to_string()),
        TokenType::LeftBrace => Some("{".to_string()),
        TokenType::RightBrace => Some("}".to_string()),
        TokenType::Comma => Some(",".to_string()),
        TokenType::Dot => Some(".".to_string()),
        TokenType::Minus => Some("-".to_string()),
        TokenType::Plus => Some("+".to_string()),
        TokenType::Semicolon => Some(";".to_string()),
        TokenType::Slash => Some("/".to_string()),
        TokenType::Star => Some("*".to_string()),

        // One or two character tokens.
        TokenType::Bang => Some("!".to_string()),
        TokenType::BangEqual => Some("!=".to_string()),
        TokenType::Equal => Some("=".to_string()),
        TokenType::EqualEqual => Some("==".to_string()),
        TokenType::Greater => Some(">".to_string()),
        TokenType::GreaterEqual => Some(">=".to_string()),
        TokenType::Less => Some("<".to_string()),
        TokenType::LessEqual => Some("<=".to_string()),

        _ => None
    }
}

fn get_token_type(lexeme: &str) -> Option<TokenType> {
    match lexeme {
        "(" => Some(TokenType::LeftParen),
        ")" => Some(TokenType::RightParen),
        "{" => Some(TokenType::LeftBrace),
        "}" => Some(TokenType::RightBrace),
        "," => Some(TokenType::Comma),
        "." => Some(TokenType::Dot),
        "-" => Some(TokenType::Minus),
        "+" => Some(TokenType::Plus),
        ";" => Some(TokenType::Semicolon),
        "/" => Some(TokenType::Slash),
        "*" => Some(TokenType::Star),

        // One or two character tokens.
        "!" => Some(TokenType::Bang),
        "!=" => Some(TokenType::BangEqual),
        "=" => Some(TokenType::Equal),
        "==" => Some(TokenType::EqualEqual),
        ">" => Some(TokenType::Greater),
        ">=" => Some(TokenType::GreaterEqual),
        "<" => Some(TokenType::Less),
        "<=" => Some(TokenType::LessEqual),

        _ => None
    }
}

fn get_token_from_keyword(keyword: String) -> TokenType {
    match keyword.as_str() {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "fun" => TokenType::Fun,
        "for" => TokenType::For,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}
