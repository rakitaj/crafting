// Questions
// 1. How do I represent a tree of different types in rust?

use crate::tokens::TokenType;

pub enum Expr {
    Binary(Box<Expr>, TokenType, Box<Expr>),
    Grouping(Box<Expr>),
    LiteralBool(bool),
    LiteralNumber(f32),
    LiteralNull,
    LiteralString(String),
    Unary(TokenType, Box<Expr>)
}

pub struct Ast {
    node: Expr
}



