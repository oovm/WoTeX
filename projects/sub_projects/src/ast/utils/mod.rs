use super::*;

mod list;

pub use self::list::List;

impl ASTNode {
    pub fn is_symbol(&self) -> bool {
        matches!(self, ASTNode::Variable(_))
    }
    pub fn is_list(&self) -> bool {
        matches!(self, ASTNode::List(_))
    }
    pub fn as_list(self) -> Option<List> {
        match self {
            ASTNode::List(list) => Some(list),
            _ => None,
        }
    }
}

impl WoTeXFunction for ASTNode {}
impl WoTeXFunction for Sequence {}
impl WoTeXFunction for MaybeParentheses {}
