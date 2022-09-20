use crate::core::errors::LoxError;
use crate::parser::{Expr, Literal};
use crate::tokens::TokenType;
use crate::value::Value;

pub struct Interpreter {
    root_expr: Expr
}

impl Interpreter {
    pub fn new(root_expr: Expr) -> Self {
        Interpreter { root_expr }
    }

    pub fn visit_expr(&self, expr: Expr) -> Result<Value, LoxError> {
        match expr {
            Expr::Literal(_, literal) => {
                match literal {
                    Literal::Nil => Ok(Value::Nil),
                    Literal::False => Ok(Value::False),
                    Literal::True => Ok(Value::True),
                    Literal::Number(number) => Ok(Value::Number(number)),
                    Literal::String(string) => Ok(Value::String(string)),
                }
            },
            Expr::Grouping(grouping) => self.visit_expr(*grouping),
            Expr::Unary(location, operator, unary) => {
                let right: Value = self.visit_expr(*unary)?;
                match operator {
                    TokenType::Minus => {
                        match right {
                            Value::Number(number) => Ok(Value::Number(-number)),
                            _ =>  {
                                let error_msg = format!("Expected Value::Number and got {:?}", right);
                                Err(LoxError::RuntimeError(location, error_msg))
                            }
                        }
                    },
                    TokenType::Bang => {
                        // Negate, aka flip, the result of is_truthy because this is the bang unary operator.
                        match right.is_truthy() {
                            true => Ok(Value::False),
                            false => Ok(Value::True)
                        }
                    },
                    _ => Err(LoxError::RuntimeError(location, "Visiting unary expr and wasn't a unary operator.".to_string()))
                }
            },
            // Expr::Binary(left_expr, operator, right_expr) => {
            //     let left = self.visit_expr(*left_expr)?;
            //     let right = self.visit_expr(*right_expr)?;
            //     match operator {
            //         TokenType::Minus => {
            //             match (left, right) {
            //                 (Value::Number(left_num), Value::Number(right_num)) => Ok(Value::Number(left_num - right_num)),
            //                 (left, right) => Err(InterpreterError::ExprBinaryMismatch(left, right, "Expected two numbers.".to_string()))
            //             }
            //         },
            //         x => Err(InterpreterError::ExprBinaryMismatch)
            //     }
            // },
            _ => Err(LoxError::Critical("Shouldn't get here".to_string()))
        }
    }
}
