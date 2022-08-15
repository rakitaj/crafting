use crate::tokens::{Token, TokenType};
use crate::Ast::{Expr, Ast};

pub struct ParserV2 {
    tokens: Vec<Token>,
    current: usize
}

impl ParserV2 {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens: tokens, current: 0 }
    }
}