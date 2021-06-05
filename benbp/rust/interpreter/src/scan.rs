use std::iter::Peekable;

use crate::token::Token;
use crate::token::TokenType;

pub struct SourceContext {
    source: String,
    idx: usize,
    line: i32,
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
    let mut curr = ctx.source.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((idx, c)) = curr.next() {
        match c {
            // Basics
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

            // Lookaheads
            '!' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        tokens.push(Token::new(TokenType::BangEqual, "!=", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Bang, "!", ctx.line, None)),
                };
            }
            '=' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        tokens.push(Token::new(TokenType::EqualEqual, "==", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Equal, "=", ctx.line, None)),
                };
            }
            '<' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        tokens.push(Token::new(TokenType::LessEqual, "<=", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Less, "<", ctx.line, None)),
                };
            }
            '>' => {
                match curr.peek() {
                    Some((_, '=')) => {
                        curr.next();
                        tokens.push(Token::new(TokenType::GreaterEqual, ">=", ctx.line, None));
                    }
                    _ => tokens.push(Token::new(TokenType::Greater, ">", ctx.line, None)),
                };
            }
            '/' => {
                match curr.peek() {
                    Some((_, '/')) => {
                        curr.next();
                        while let Some((_, next_c)) = curr.peek() {
                            match next_c {
                                '\n' => break,
                                _ => {
                                    curr.next();
                                    continue;
                                }
                            }
                        }
                    }
                    _ => tokens.push(Token::new(TokenType::Slash, "/", ctx.line, None)),
                };
            }

            // Misc
            ' ' | '\r' | '\t' => {}
            '\n' => ctx.line += 1,
            '0'..='9' => {
                let twcurr = &mut curr;
                let tw: String = TakeUntil {
                    inner: twcurr,
                    condition: |(_, d)| d.is_digit(10),
                }
                .map(|(_, d)| d)
                .collect();

                println!("\ndigits!: {}", tw);
                //let number: String = curr
                //    .take_while(|(_, d)| d.is_digit(10) || *d == '.')
                //    .map(|(_, d)| d)
                //    .collect();
                //if number.contains('.') {
                //} else {
                //}
                //tokens.push(Token::new(TokenType::Number, "", ctx.line, Option<number>));
            }
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
struct TakeUntil<'a, T: Iterator + 'a, P: FnMut(&T::Item) -> bool>
where
    T::Item: 'a,
{
    inner: &'a mut Peekable<T>,
    condition: P,
}

impl<'a, T: Iterator, P> Iterator for TakeUntil<'a, T, P>
where
    P: FnMut(&T::Item) -> bool,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<T::Item> {
        let return_next = match self.inner.peek() {
            Some(ref v) => (self.condition)(v),
            _ => false,
        };
        if return_next {
            self.inner.next()
        } else {
            None
        }
    }
}

/*
fn foo<'a, T: Iterator, P: FnMut(&T::Item) -> bool>(t: T, p: P) -> TakeUntil<'a, T, P> {
    return TakeUntil {
        inner: t,
        condition: p,
    };
}
*/

trait TakeUntilable<'a, T: Iterator>: Iterator {
    fn take_until<P: FnMut(&T::Item) -> bool>(&'a mut self, f: P) -> TakeUntil<'a, T, P>;
    //    fn take_until<P: FnMut(&Self::Item)>(&'a mut self, f: P) -> TakeUntil<'a, T, P>;
    //    ) -> TakeUntil<'a, T: Iterator, P: FnMut(&T::Item)>;
    //fn take_until<P: FnMut(&Self::Item) -> bool>>(&'a mut self, P) -> TakeUntil<'a, T: Iterator, P>;
    //    fn cautious_take_while<P>(&'a mut self, P) -> TakeUntil<'a, T: Iterator + 'a, P> where
    //        P: FnMut(&Self::Item) -> bool;
}

impl<'a, T: Iterator> TakeUntilable<'a, T> for Peekable<T> {
    fn take_until<P: FnMut(&T::Item) -> bool>(&'a mut self, f: P) -> TakeUntil<'a, T, P> {
        TakeUntil {
            inner: self,
            condition: f,
        }
    }
}

/*
impl<'a, T: Iterator> TakeUntilable<'a, T> for Peekable<T> {
    fn cautious_take_while<P>(&'a mut self, f: P) -> TakeUntil<'a, T, P> where
        P: FnMut(&'a(T::Item)) -> bool {
                TakeUntil{inner:  self, condition: f}
        }
}
*/
