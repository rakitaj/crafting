use crate::token;

macro_rules! ast_type {
    ($type_name:ident, $($fname: ident, $ftype: ty),+) => {
        #[derive(Debug)]
        pub struct $type_name {
            $(pub $fname: $ftype),+
        }
    }
}

#[derive(Debug)]
pub struct Expr {
}

#[derive(Debug)]
pub struct Token {
}

ast_type!(Binary, left, Expr, operator, Token, right, Expr);
ast_type!(Grouping, expression, Expr);
ast_type!(Literal, value, token::Literal);
ast_type!(Unary, operator, Token, right, Expr);
