use crate::parser::{Expr, Literal};
use crate::tokens::TokenType;
use crate::value::Value;

pub struct Interpreter {
    root_expr: Expr
}

pub enum InterpreterError {
    ValueTypeError(Value, String),
    ExprUnaryMismatch(TokenType, String),
    ExprBinaryMismatch,
    GenericError(String)
}

impl Interpreter {
    pub fn new(root_expr: Expr) -> Self {
        Interpreter { root_expr }
    }

    pub fn visit_expr(&self, expr: Expr) -> Result<Value, InterpreterError> {
        match expr {
            Expr::Literal(literal) => {
                match literal {
                    Literal::Nil => Ok(Value::Nil),
                    Literal::False => Ok(Value::False),
                    Literal::True => Ok(Value::True),
                    Literal::Number(number) => Ok(Value::Number(number)),
                    Literal::String(string) => Ok(Value::String(string)),
                }
            },
            Expr::Grouping(grouping) => self.visit_expr(*grouping),
            Expr::Unary(operator, unary) => {
                let right: Value = self.visit_expr(*unary)?;
                match operator {
                    TokenType::Minus => {
                        match right {
                            Value::Number(number) => Ok(Value::Number(-number)),
                            _ =>  {
                                let error_msg = format!("Expected Value::Number and got {:?}", right);
                                Err(InterpreterError::ValueTypeError(right, error_msg))
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
                    _ => Err(InterpreterError::ExprUnaryMismatch(operator, "Visiting Unary".to_string()))
                }
            },
            Expr::Binary(left_expr, operator, right_expr) => {
                let left = self.visit_expr(*left_expr)?;
                let right = self.visit_expr(*right_expr)?;
                match operator {
                    TokenType::Minus => {
                        match (left, right) {
                            (Value::Number(left_num), Value::Number(right_num)) => Ok(Value::Number(left_num - right_num)),
                            (left, right) => Err(InterpreterError::ExprBinaryMismatch)
                        }
                    },
                    x => Err(InterpreterError::ExprBinaryMismatch)
                }
            },
            _ => Err(InterpreterError::GenericError("Shouldn't get here".to_string()))
        }
    }
}
