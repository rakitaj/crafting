use crate::parser::{Expr, Literal};
use crate::value::Value;

pub struct Interpreter {
    root_expr: Expr
}

pub enum InterpreterError {
    Blah
}

impl Interpreter {
    pub fn new(root_expr: Expr) -> Self {
        Interpreter { root_expr }
    }

    pub fn visit_expr(&self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(literal) => {
                match literal {
                    Literal::Nil => Value::Nil,
                    Literal::False => Value::False,
                    Literal::True => Value::True,
                    Literal::Number(number) => Value::Number(number),
                    Literal::String(string) => Value::String(string),
                }
            },
            Expr::Grouping(grouping) => self.visit_expr(*grouping),
            Expr::Unary(token_type, unary) => {
                let right: Value = self.visit_expr(*unary);
                match token_type {
                    TokenType::Minus => 
                }
            }
        }
    }
}
