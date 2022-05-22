use super::*;

impl ASTNode {
    pub fn is_symbol(&self) -> bool {
        matches!(self, ASTNode::Symbol(_))
    }
    pub fn is_list(&self) -> bool {
        matches!(self, ASTNode::List(_))
    }
}


impl WoTeXFunction for ASTNode {}
impl WoTeXFunction for Sequence {}
impl WoTeXFunction for MaybeParentheses {}