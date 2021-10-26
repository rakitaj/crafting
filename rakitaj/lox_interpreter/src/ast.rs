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

fn print_ast(ast: Ast) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    // print_ast_helper(Some(&ast.node), &result);
    return result;
}

// fn print_ast_helper(node: Option<&Expr>, mut result: &Vec<String>) {
//     match node {
//         Some(expr) => match expr {
//             Expr::Binary(left, op, right) => format!()
//         },
//         None => {}
//     }
// }