use std::env;
use std::fs;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let program = fs::read_to_string(filename)
        .expect(&format!("Error reading file {}", filename));

    tokenize(program);
}

fn tokenize(program: String) {
    println!("{}", program)
}

impl<T> fmt::Display for Token<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

#[allow(dead_code)]
struct Token<T> {
    token_type: TokenType,
    lexeme: String,
    literal: T,
    line: i32
}

#[allow(dead_code)]
enum TokenType {
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
