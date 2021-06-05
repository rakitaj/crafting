use crate::token::Token;
use crate::token::TokenType;

pub struct SourceContext {
    source: String,
    idx: usize,
    line: i32,
}

impl SourceContext {
    fn next() -> char {
        let foo: char = 'a';
        foo
    }
}

pub fn new_source(program: String) -> SourceContext {
    SourceContext {
        source: program,
        idx: 0,
        line: 0,
    }
}

pub fn tokenize(source: SourceContext) {
    scan_token(source)
}

fn scan_token(source: SourceContext) {
    let mut ctx = source;
    let mut curr = ctx.source.char_indices();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((idx, c)) = curr.next() {
        match c {
            '(' => tokens.push(Token::new(TokenType::LeftParen, "(", ctx.line, None)),
            ')' => tokens.push(Token::new(TokenType::RightParen, ")", ctx.line, None)),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, "{", ctx.line, None)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, "}", ctx.line, None)),
            ',' => tokens.push(Token::new(TokenType::Comma, ",", ctx.line, None)),
            '.' => tokens.push(Token::new(TokenType::Dot, ".", ctx.line, None)),
            '_' => tokens.push(Token::new(TokenType::Minus, "_", ctx.line, None)),
            '+' => tokens.push(Token::new(TokenType::Plus, "+", ctx.line, None)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, ";", ctx.line, None)),
            '*' => tokens.push(Token::new(TokenType::Star, "*", ctx.line, None)),

            // Add peek for combos
            '!' => {
                // TODO: pass reference to curr down to func, so we can set curr to the curr clone
                // if there is a match, as we need to increment past the matched lookahead
                //let ttype = type_from_lookahead(curr.clone().next(), '=', TokenType::BangEqual, TokenType::Bang);
                // TODO: lexeme needs to encompass all characters
                //tokens.push(Token::new(ttype, "", ctx.line, None))

                let mut clone = curr.clone();
                let next = clone.next();
                match next {
                    Some((_, '=')) => {
                        curr = clone; // advance iterator past lexeme
                        tokens.push(Token::new(TokenType::BangEqual, "!=", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Bang, "!", ctx.line, None)),
                };
            }

            '\n' => ctx.line += 1,
            _ => {
                ctx.idx = idx;
                // println!("DEFAULT {} at {}", c, idx)
                print!("{}", c);
            }
        }
    }

    for t in tokens {
        println!("t: {}", t.lexeme);
    }

    println!("");
}

fn type_from_lookahead(
    peek_val: Option<(usize, char)>,
    comparator: char,
    match_type: TokenType,
    no_match_type: TokenType,
) -> TokenType {
    match peek_val {
        Some((_, peek_char)) => {
            if peek_char == comparator {
                match_type
            } else {
                no_match_type
            }
        }
        _ => no_match_type,
    }
}
