use std::fmt;

impl<T> fmt::Display for Token<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

#[allow(dead_code)]
pub struct Token<T> {
    token_type: TokenType,
    lexeme: String,
    literal: T,
    line: i32
}

#[allow(dead_code)]
pub enum TokenType {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, StringLiteral, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  EOF
}
