use crate::parser::{Parser, Expr};

// //pub trait Visitor<T> {
// pub trait Visitor {
//     fn visit_literal_bool(&self, literal: &Expr) -> bool;
//     fn visit_literal_nil(&self, literal: &Expr) -> None;
//     fn visit_literal_number(&self, literal: &Expr) -> f32;
//     fn visit_literal_string(&self, literal: &Expr) -> String;
// }

// trait Data {
//     fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Result;
// }
