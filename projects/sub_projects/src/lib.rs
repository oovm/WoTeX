mod errors;
mod ast;
mod vm;


pub use errors::{Error, Result};


pub enum ASTNode {
    Function(String, Vec<ASTNode>),
}

