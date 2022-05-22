use std::fmt::{Formatter, LowerHex};

mod predefined;
mod as_tex;


pub enum ASTNode {
    Function(String, Vec<ASTNode>),
    Symbol(String),
    Integrate(Vec<ASTNode>),
    MaybeParentheses(MaybeParentheses),
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
    rest: ASTNode,
}

#[test]
fn test() {
    println!("1")
}