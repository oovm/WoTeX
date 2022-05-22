use super::*;


impl UpperHex for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Sequence(_) => { todo!() }
            ASTNode::List(_) => { todo!() }
            ASTNode::Variable(_) => { todo!() }
            ASTNode::Function(_, _) => { todo!() }
            ASTNode::Evaluated(e) => { LowerHex::fmt(&**e, f) }
        }
    }
}

impl UpperHex for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl UpperHex for MaybeParentheses {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}