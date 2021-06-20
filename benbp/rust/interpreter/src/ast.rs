use crate::token;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, token::Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(token::Literal),
    Unary(token::Token, Box<Expr>)
}

macro_rules! parenthesize {
    ($s:expr, $name:expr, $($_expr:expr),+) => {
        $s = format!("{}{}", $s, $name);
        $($s = format!("{} {}", $s, format!("{}", format_ast(*$_expr)));)+
    }
}

pub fn format_ast(expr: Expr) -> String {
    let mut s = "(".to_string();
    match expr {
        Expr::Binary(left, operator, right) => {
            parenthesize!(s, operator.lexeme.as_str(), left, right);
        }
        Expr::Grouping(e) => {
            parenthesize!(s, "group", e);
        }
        Expr::Literal(literal) => {
            s = format!("{}{}", s, literal);
        }
        Expr::Unary(operator, e) => {
            parenthesize!(s, operator.lexeme.as_str(), e);
        }
    }
    format!("{})", s)
}
