mod ast;

pub use ast::*;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar, "/ql/grammar.rs");

pub struct Context {
    pub date: chrono::NaiveDate,
}

pub trait Evaluate<T> {
    fn evaluate(&self, context: &Context) -> T;
}
