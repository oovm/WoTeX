use super::*;

impl UpperHex for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTNode::Sequence(e) => UpperHex::fmt(e, f),
            ASTNode::List(e) => UpperHex::fmt(e, f),
            ASTNode::Variable(e) => Display::fmt(e, f),
            ASTNode::Function(_, _) => {
                todo!()
            }
            ASTNode::Evaluated(e) => UpperHex::fmt(&**e, f),
        }
    }
}

impl UpperHex for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl UpperHex for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl UpperHex for MaybeParentheses {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let head = match self.head.as_str() {
            "sin" => "Sin",
            "cos" => "Cos",
            "tan" => "Tan",
            "cot" => "Cot",
            "sec" => "Sec",
            "csc" => "Csc",
            s => s,
        };
        write!(f, "{}[]", head)
    }
}

impl UpperHex for Integrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
