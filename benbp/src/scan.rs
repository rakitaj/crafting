use std::iter::Peekable;
use std::str::CharIndices;

use crate::iter_extensions::TakeWhileExclusiveable;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;

pub struct SourceContext {
    line: i32,
    offset: usize,
    pub had_error: bool,
}

fn report_error(line: i32, col: usize, message: String) {
    println!("[line {}, col {}] Error: {}", line, col, message);
}

pub fn tokenize(program: String) -> bool {
    let ctx = &mut SourceContext {
        line: 0,
        offset: 0,
        had_error: false,
    };
    let curr = &mut program.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((idx, c)) = curr.next() {
        let token = match c {
            // ----------- Basics -----------
            '(' => Token::new(TokenType::LeftParen, ctx.line, None),
            ')' => Token::new(TokenType::RightParen, ctx.line, None),
            '{' => Token::new(TokenType::LeftBrace, ctx.line, None),
            '}' => Token::new(TokenType::RightBrace, ctx.line, None),
            ',' => Token::new(TokenType::Comma, ctx.line, None),
            '.' => Token::new(TokenType::Dot, ctx.line, None),
            '-' => Token::new(TokenType::Minus, ctx.line, None),
            '+' => Token::new(TokenType::Plus, ctx.line, None),
            ';' => Token::new(TokenType::Semicolon, ctx.line, None),
            '*' => Token::new(TokenType::Star, ctx.line, None),
            // ----------- Lookaheads -----------
            '!' => match_lexeme_and_advance(curr, c, '=', ctx.line),
            '=' => match_lexeme_and_advance(curr, c, '=', ctx.line),
            '<' => match_lexeme_and_advance(curr, c, '=', ctx.line),
            '>' => match_lexeme_and_advance(curr, c, '=', ctx.line),
            '/' => match_comment(curr, ctx.line),
            // ----------- Misc -----------
            ' ' | '\r' | '\t' => None,
            '\n' => {
                ctx.line += 1;
                ctx.offset = idx;
                None
            }
            // ----------- Literals -----------
            '0'..='9' => match_numeric(curr, c, ctx.line),
            '"' => match_string_literal(curr, ctx),
            'a'..='z' | 'A'..='Z' => match_alphanumeric(curr, c, ctx.line),
            _ => {
                ctx.had_error = true;
                report_error(
                    ctx.line,
                    idx - ctx.offset,
                    format!("Unexpected character {}", c),
                );
                None
            }
        };
        if let Some(token) = token { tokens.push(token) }
    }

    if let Some(eof) = Token::new_as_type(TokenType::Eof, "".to_string(), ctx.line, None) {
        tokens.push(eof);
    }

    for t in tokens {
        println!("{:?}", t);
    }

    ctx.had_error
}

fn match_lexeme_and_advance(
    iter: &mut Peekable<CharIndices>,
    first: char,
    second: char,
    line: i32,
) -> Option<Token> {
    match iter.peek() {
        Some((_, c)) if *c == second => {
            iter.next();
            Token::new_from_lexeme(format!("{}{}", first, second).as_str(), line, None)
        }
        // TODO: easier char to &str conversion?
        _ => Token::new_from_lexeme(first.to_string().as_str(), line, None),
    }
}

// TODO: refactor match as separate function from peek+while so that functionality can be shared
fn match_comment(iter: &mut Peekable<CharIndices>, line: i32) -> Option<Token> {
    match iter.peek() {
        Some((_, '/')) => {
            iter.next();
            while let Some((_, next_c)) = iter.peek() {
                match next_c {
                    '\n' => break,
                    _ => {
                        iter.next();
                        continue;
                    }
                }
            }
            None
        }
        _ => Token::new(TokenType::Slash, line, None),
    }
}

fn match_numeric(iter: &mut Peekable<CharIndices>, first: char, line: i32) -> Option<Token> {
    let rest: String = iter
        .take_while_exclusive(|(_, d)| d.is_numeric() || *d == '.')
        .map(|(_, d)| d)
        .collect();
    let lexeme = format!("{}{}", first, rest);
    let number: f64 = lexeme.parse().unwrap();
    Token::new_as_type(
        TokenType::Number,
        lexeme.clone(),
        line,
        Some(Literal::Number(number)),
    )
}

fn match_alphanumeric(iter: &mut Peekable<CharIndices>, first: char, line: i32) -> Option<Token> {
    let rest: String = iter
        .take_while_exclusive(|(_, s)| s.is_alphanumeric())
        .map(|(_, s)| s)
        .collect();
    let lexeme = format!("{}{}", first, rest);
    Token::new_as_keyword(
        lexeme.clone(),
        line,
        Some(Literal::Identifier(lexeme)),
    )
}

fn match_string_literal(
    iter: &mut Peekable<CharIndices>,
    ctx: &mut SourceContext,
) -> Option<Token> {
    let literal: String = iter
        .take_while_exclusive(|(_, s)| *s != '"' && *s != '\n')
        .map(|(_, s)| s)
        .collect();
    if let Some(next) = iter.next() {
        match next {
            (_, '\n') => {
                ctx.line += 1;
                ctx.had_error = true;
                report_error(
                    ctx.line,
                    next.0 - ctx.offset,
                    "Unterminated string".to_string(),
                );
                None
            }
            _ => Token::new_as_type(
                TokenType::StringLiteral,
                format!("\"{}\"", literal),
                ctx.line,
                Some(Literal::String(literal)),
            ),
        }
    } else {
        None
    }
}
