use std::io::Write;

use crate::core::errors::LoxError;
use crate::environment::Environment;
use crate::parser::{Expr, Literal, Stmt};
use crate::tokens::TokenType;
use crate::value::Value;

pub struct InterpreterState<W: Write> {
    environment: Environment,
    writer: W,
}

impl Default for InterpreterState<std::io::Stdout> {
    fn default() -> Self {
        InterpreterState {
            environment: Environment::new(),
            writer: std::io::stdout(),
        }
    }
}

impl Default for InterpreterState<Vec<u8>> {
    fn default() -> Self {
        InterpreterState {
            environment: Environment::new(),
            writer: Vec::new(),
        }
    }
}

impl InterpreterState<Vec<u8>> {
    pub fn get_writer(&self) -> &str {
        std::str::from_utf8(&self.writer).unwrap()
    }
}

pub struct Interpreter {
    statements: Vec<Stmt>,
}

impl Interpreter {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Interpreter { statements }
    }

    pub fn interpret<T: Write>(&self, state: &mut InterpreterState<T>) -> Vec<LoxError> {
        let mut errors: Vec<LoxError> = Vec::new();
        for stmt in &self.statements {
            let result = self.evaluate(stmt, state);
            match result {
                Ok(_) => (),
                Err(err) => errors.push(err),
            }
        }
        errors
    }

    fn evaluate<T: Write>(
        &self,
        stmt: &Stmt,
        state: &mut InterpreterState<T>,
    ) -> Result<Option<Value>, LoxError> {
        match stmt {
            Stmt::Expression(expr) => {
                let value = self.evaluate_expr(expr, state)?;
                Ok(Some(value))
            }
            Stmt::Print(expr) => self.evaluate_print(expr, state),
            Stmt::Var(identifier_token, initializer) => {
                if let TokenType::Identifier(name) = &identifier_token.token_type {
                    let value = self.evaluate_expr(initializer, state)?;
                    state.environment.define(name.to_string(), value);
                    Ok(None)
                } else {
                    Err(LoxError::RuntimeError(
                        identifier_token.location.clone(),
                        format!(
                            "Expected identifier token for var name. {}",
                            identifier_token
                        ),
                    ))
                }
            }
            Stmt::Block(statements) => {
                state.environment.new_child_scope();
                self.execute_block(statements, state)?;
                state.environment.destroy_child_scope();
                Ok(None)
            }
            Stmt::If(condition, left_stmt, right_stmt) => {
                match self.evaluate_expr(condition, state)?.is_truthy() {
                    true => self.evaluate(left_stmt, state),
                    false => match right_stmt {
                        Some(x) => self.evaluate(x, state),
                        None => Ok(None),
                    },
                }
            }
            Stmt::While(condition, body) => {
                while self.evaluate_expr(condition, state)?.is_truthy() {
                    self.evaluate(body, state)?;
                }
                Ok(None)
            }
        }
    }

    fn execute_block<T: Write>(
        &self,
        statements: &Vec<Stmt>,
        state: &mut InterpreterState<T>,
    ) -> Result<(), LoxError> {
        for stmt in statements {
            self.evaluate(stmt, state)?;
        }
        Ok(())
    }

    fn evaluate_print<T: Write>(
        &self,
        expr: &Expr,
        state: &mut InterpreterState<T>,
    ) -> Result<Option<Value>, LoxError> {
        let value = self.evaluate_expr(expr, state)?;
        // writeln!(state.writer, "{}", value);
        if let Err(err) = writeln!(state.writer, "{}", value) {
            let wrapped_syscall_error = LoxError::new_syscall(std::file!(), 111, err.to_string());
            return Err(wrapped_syscall_error);
        }
        Ok(None)
    }

    fn evaluate_expr<T: Write>(
        &self,
        expr: &Expr,
        state: &mut InterpreterState<T>,
    ) -> Result<Value, LoxError> {
        match expr {
            Expr::Literal(_, literal) => match literal {
                Literal::Nil => Ok(Value::Nil),
                Literal::False => Ok(Value::Boolean(false)),
                Literal::True => Ok(Value::Boolean(true)),
                Literal::Number(number) => Ok(Value::Number(*number)),
                Literal::String(string) => Ok(Value::String(string.to_string())),
            },
            Expr::Grouping(grouping) => self.evaluate_expr(grouping, state),
            Expr::Unary(operator, unary) => {
                let right: Value = self.evaluate_expr(unary, state)?;
                match operator.token_type {
                    TokenType::Minus => match right {
                        Value::Number(number) => Ok(Value::Number(-number)),
                        _ => {
                            let error_msg = format!("Expected Value::Number and got {:?}", right);
                            Err(LoxError::RuntimeError(operator.location.clone(), error_msg))
                        }
                    },
                    TokenType::Bang => {
                        // Negate, aka flip, the result of is_truthy because this is the bang unary operator.
                        match right.is_truthy() {
                            true => Ok(Value::Boolean(false)),
                            false => Ok(Value::Boolean(true)),
                        }
                    }
                    _ => Err(LoxError::RuntimeError(
                        operator.location.clone(),
                        "Visiting unary expr and wasn't a unary operator.".to_string(),
                    )),
                }
            }
            Expr::Binary(left_expr, operator, right_expr) => {
                let left = self.evaluate_expr(left_expr, state)?;
                let right = self.evaluate_expr(right_expr, state)?;
                match (&left, &right, &operator.token_type) {
                    (left, right, TokenType::EqualEqual) => Ok(Value::Boolean(left == right)),
                    (left, right, TokenType::BangEqual) => Ok(Value::Boolean(!(left == right))),
                    (Value::Number(left_num), Value::Number(right_num), token_type) => {
                        match token_type {
                            TokenType::Minus => Ok(Value::Number(left_num - right_num)),
                            TokenType::Slash => Ok(Value::Number(left_num / right_num)),
                            TokenType::Star => Ok(Value::Number(left_num * right_num)),
                            TokenType::Plus => Ok(Value::Number(left_num + right_num)),
                            TokenType::Greater => Ok(Value::Boolean(left_num > right_num)),
                            TokenType::GreaterEqual => Ok(Value::Boolean(left_num >= right_num)),
                            TokenType::Less => Ok(Value::Boolean(left_num < right_num)),
                            TokenType::LessEqual => Ok(Value::Boolean(left_num <= right_num)),
                            _ => Err(LoxError::RuntimeError(
                                operator.location.clone(),
                                "Matching number. Should be unreachable".to_string(),
                            )),
                        }
                    }
                    (Value::String(left_string), Value::String(right_string), token_type) => {
                        match token_type {
                            TokenType::Plus => {
                                let concat_string = format!("{}{}", left_string, right_string);
                                Ok(Value::String(concat_string))
                            }
                            _ => Err(LoxError::RuntimeError(
                                operator.location.clone(),
                                "Matching string. Should be unreachable".to_string(),
                            )),
                        }
                    }
                    _ => {
                        let msg = format!(
                            "Expected two numbers and got left: {:?} -- right: {:?}",
                            left, right
                        );
                        Err(LoxError::RuntimeError(operator.location.clone(), msg))
                    }
                }
            }
            Expr::Logical(left_expr, operator, right_expr) => {
                let left = self.evaluate_expr(left_expr, state)?;
                match (&operator.token_type, left.is_truthy()) {
                    (TokenType::Or, true) => Ok(left),
                    (TokenType::Or, false) => self.evaluate_expr(right_expr, state),
                    (TokenType::And, true) => self.evaluate_expr(right_expr, state),
                    (TokenType::And, false) => Ok(left),
                    (_, _) => Err(LoxError::SyntaxError(
                        operator.location.clone(),
                        format!("Can't interpret conditional with operator: {}", operator),
                    )),
                }
            }
            Expr::Variable(token) => match &token.token_type {
                TokenType::Identifier(variable_name) => {
                    match state.environment.get(variable_name) {
                        Some(val) => Ok(val),
                        None => Err(LoxError::RuntimeError(
                            token.location.clone(),
                            format!("Undefined variable: {}", variable_name),
                        )),
                    }
                }
                _ => Err(LoxError::RuntimeError(
                    token.location.clone(),
                    format!("Expected a variable expression. Got {}", token),
                )),
            },
            Expr::Assign(token, expr) => {
                let value = self.evaluate_expr(expr, state)?;
                match &token.token_type {
                    TokenType::Identifier(name) => {
                        state
                            .environment
                            .assign(name, value.clone(), token.location.clone())?;
                        Ok(value)
                    }
                    _ => Err(LoxError::RuntimeError(
                        token.location.clone(),
                        "Assignment didn't work".to_string(),
                    )),
                }
            }
            _ => Err(LoxError::Critical(
                "Happening in the interpreter.".to_string(),
            )),
        }
    }
}
