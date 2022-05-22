use std::fmt::{Formatter, LowerHex, UpperHex};

mod predefined;
mod as_tex;
mod utils;


pub enum ASTNode {
    Sequence(Vec<ASTNode>),
    List(Vec<ASTNode>),
    Symbol(String),
    Function(String, Vec<ASTNode>),
    Evaluated(Box<dyn WoTeXFunction>),
}

pub trait WoTeXFunction: LowerHex + UpperHex {
    fn is_simple(&self) -> bool;
}

pub struct Integrate {
    pub var: String,
    pub expr: ASTNode,
}


/// Functions that can be added with or without parentheses
///
/// eg: sin cos tan arctan
pub struct MaybeParentheses {
    head: String,
    operator_name: bool,
    rest: Vec<ASTNode>,
}

#[test]
fn test() {
    println!("1")
}