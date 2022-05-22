use std::fmt::{Formatter, LowerHex};
use crate::ASTNode;

pub struct WoTeXRuntime {}

pub trait WoTeXFunction {
    fn as_tex(&self) -> String;
    fn as_wolfram(&self) -> String;
}


