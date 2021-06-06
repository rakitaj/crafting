use std::iter::Peekable;
use std::str::CharIndices;

use crate::iter_extensions::TakeWhileExclusiveable;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;

pub struct SourceContext {
    line: i32,
    pub had_error: bool
}

// TODO: track offset incrementally to derive column in line
fn report_error(line: i32, offset: usize, message: String) {
    println!("[line {}, offset {}] Error: {}", line, offset, message);
}

pub fn tokenize(program: String) -> bool {
    let ctx = &mut SourceContext {
        line: 0,
        had_error: false,
    };
    let curr = &mut program.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((idx, c)) = curr.next() {
        let token = match c {
            // ----------- Basics -----------
            '(' => Token::new("(", ctx.line, None),
            ')' => Token::new(")", ctx.line, None),
            '{' => Token::new("}", ctx.line, None),
            '}' => Token::new("{", ctx.line, None),
            ',' => Token::new(",", ctx.line, None),
            '.' => Token::new(".", ctx.line, None),
            '_' => Token::new("_", ctx.line, None),
            '+' => Token::new("+", ctx.line, None),
            ';' => Token::new(";", ctx.line, None),
            '*' => Token::new("*", ctx.line, None),
            // ----------- Lookaheads -----------
            '!' => { match_lexeme_and_advance(curr, c, '=', ctx.line) }
            '=' => { match_lexeme_and_advance(curr, c, '=', ctx.line) }
            '<' => { match_lexeme_and_advance(curr, c, '=', ctx.line) }
            '>' => { match_lexeme_and_advance(curr, c, '=', ctx.line) }
            '/' => { match_comment(curr, ctx.line) }
            // ----------- Misc -----------
            ' ' | '\r' | '\t' => { None }
            '\n' => { ctx.line += 1; None },
            // ----------- Literals -----------
            '0'..='9' => { match_numeric(curr, c, ctx.line) }
            '"' => { match_string_literal(curr, ctx) }
            'a'..='z' | 'A'..='Z' => { match_alphanumeric(curr, c, ctx.line) }
            _ => {
                ctx.had_error = true;
                report_error(ctx.line, idx, format!("Unexpected character {}", c));
                None
            }
        };
        match token {
            Some(token) => { tokens.push(token) }
            _ => {}
        };
    }

    if let Some(eof) = Token::new_as_type(TokenType::EOF, "".to_string(), ctx.line, None) {
        tokens.push(eof);
    }

    for t in tokens {
        println!("{:?}", t);
    };

    ctx.had_error
}

fn match_lexeme_and_advance(iter: &mut Peekable<CharIndices>, first: char, second: char, line: i32) -> Option<Token> {
    // TODO: find alternative and/or verify clone is cheap (I think it is, just a small Item tuple + pointer)
    match iter.clone().peek() {
        Some((_, c)) if *c == second => {
            iter.next();
            Token::new(format!("{}{}", first, second).as_str(), line, None)
        }
        // TODO: easier char to &str conversion?
        _ => Token::new(first.to_string().as_str(), line, None),
    }
}

fn match_comment(iter: &mut Peekable<CharIndices>, line: i32) -> Option<Token> {
    match iter.peek() {
        Some((_, '/')) => {
            iter.next();
            while let Some((_, next_c)) = iter.peek() {
                match next_c {
                    '\n' => break,
                    _ => { iter.next(); continue; }
                }
            }
            None
        }
        _ => Token::new("/", line, None)
    }
}

fn match_numeric(iter: &mut Peekable<CharIndices>, first: char, line: i32) -> Option<Token> {
    let rest: String = iter
        .take_while_exclusive(|(_, d)| d.is_numeric() || *d == '.')
        .map(|(_, d)| d)
        .collect();
    let lexeme = format!("{}{}", first, rest);
    let number: f64 = lexeme.parse().unwrap();
    Token::new_as_type(TokenType::Number, lexeme.clone(), line, Some(Literal::NumberLiteral(number)))
}

fn match_alphanumeric(iter: &mut Peekable<CharIndices>, first: char, line: i32) -> Option<Token> {
    let rest: String = iter
        .take_while_exclusive(|(_, s)| s.is_alphanumeric())
        .map(|(_, s)| s)
        .collect();
    let lexeme = format!("{}{}", first, rest);
    Token::new_as_keyword(lexeme.clone(), line, Some(Literal::IdentifierLiteral(lexeme)))
}

fn match_string_literal(iter: &mut Peekable<CharIndices>, ctx: &mut SourceContext) -> Option<Token> {
    let literal: String = iter
        .take_while_exclusive(|(_, s)| *s != '"' && *s != '\n')
        .map(|(_, s)| s)
        .collect();
    if let Some(next) = iter.next() {
        match next {
            (_, '\n') => {
                ctx.line += 1;
                ctx.had_error = true;
                report_error(ctx.line, next.0, format!("Unterminated string"));
                None
            }
            _ => Token::new_as_type(
                     TokenType::StringLiteral,
                     format!("\"{}\"", literal),
                     ctx.line,
                     Some(Literal::StringLiteral(literal))
                 )
        }
    } else {
        None
    }
}
