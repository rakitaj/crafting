// Questions
// 1. How do I represent a tree of different types in rust?

use crate::tokens::Token;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    LiteralBool(bool),
    LiteralNumber(f32),
    LiteralNull,
    LiteralString(String),
    Unary(Token, Box<Expr>)
}

pub struct Ast {
    pub node: Expr
}
