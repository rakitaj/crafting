use crate::tokens::TokenType;

pub enum Expr {
    Binary,
    Grouping,
    Literal,
    Unary
}

pub struct BinaryExpr {
    left: Expr,
    operator: TokenType,
    right: Expr
}

pub struct GroupingExpr {
    expression: Expr
}

pub struct LiteralExpr<T> {
    value: T
}

pub struct UnaryExpr {
    operator: TokenType,
    right: Expr
}

