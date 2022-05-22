use std::fmt::{Display, Formatter, LowerHex, UpperHex, Write};

mod as_tex;
mod as_wolfram;
mod predefined;
mod utils;

pub use self::{predefined::*, utils::List};

pub enum ASTNode {
    Sequence(Sequence),
    List(List),
    Variable(String),
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

impl Default for Sequence {
    fn default() -> Self {
        Self { inner: vec![] }
    }
}

pub struct Integrate {
    pub kind: String,
    pub regional: bool,
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
    let seq = Sequence { inner: vec![ASTNode::Variable("a".to_string()), ASTNode::Variable("b".to_string())] };
    let sin = TrigonometricFunction::sin(seq);
    let ast = ASTNode::Evaluated(sin);
    println!("{:x}", ast);

    let expr = Sequence { inner: vec![ASTNode::Variable("x".to_string()), ASTNode::Variable("y".to_string())] };
    let vars = List::one(ASTNode::Variable("x".to_string()));
    let int = Integrate::int(expr, vec![vars], false);
    let ast = ASTNode::Evaluated(int);
    println!("{:x}", ast);
}
