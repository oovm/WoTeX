use std::fmt::{Formatter, LowerHex, UpperHex};

mod predefined;
mod as_tex;
mod utils;
mod as_wolfram;

pub use self::predefined::*;

pub enum ASTNode {
    Sequence(Sequence),
    List(List),
    Symbol(String),
    Function(String, Vec<ASTNode>),
    Evaluated(Box<dyn WoTeXFunction>),
}

pub trait WoTeXFunction: LowerHex + UpperHex {
    fn as_tex(&self) -> String {
        format!("{:x}", self)
    }
    fn as_wolfram(&self) -> String {
        format!("{:X}", self)
    }
}

pub struct Sequence {
    inner: Vec<ASTNode>,
}

pub struct List {
    inner: Vec<ASTNode>,
}

pub struct Integrate {
    pub kind: String,
    /// A sequence of ASTNodes
    pub expr: Sequence,
    pub vars: Vec<List>,
}


/// Functions that can be added with or without parentheses
///
/// eg: sin cos tan arctan
pub struct MaybeParentheses {
    head: String,
    operator_name: bool,
    rest: Vec<Sequence>,
}

#[test]
fn test() {
    let seq = Sequence {
        inner: vec![ASTNode::Symbol("a".to_string()), ASTNode::Symbol("b".to_string())],
    };
    let sin = TrigonometricFunction::sin(seq);

    let ast = ASTNode::Evaluated(sin);

    println!("{:x}", ast);
}