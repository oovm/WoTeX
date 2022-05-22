use super::*;

impl ASTNode {
    pub fn is_symbol(&self) -> bool {
        matches!(self, ASTNode::Symbol(_))
    }
    pub fn is_list(&self) -> bool {
        matches!(self, ASTNode::List(_))
    }
}