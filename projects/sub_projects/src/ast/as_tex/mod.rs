use std::fmt::Write;
use super::*;


impl LowerHex for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("x")
    }
}

impl LowerHex for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl MaybeParentheses {
    fn need_parentheses(&self) -> bool {
        // f() or atan(x, y)
        if self.rest.len() != 1 {
            return true;
        }
        false
    }
}

impl LowerHex for MaybeParentheses {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.operator_name {
            true =>
                write!(f, "\\operatorname{{{}}}", self.head)?,
            false =>
                write!(f, "\\{}", self.head)?
        }
        let parentheses = self.need_parentheses();
        let count = self.rest.len();
        f.write_char('{')?;
        if parentheses {
            f.write_str("\\left(")?;
        }
        for (i, ast) in self.rest.iter().enumerate() {
            write!(f, "{:x}", ast)?;
            if i != count - 1 {
                f.write_char(',')?;
            }
        }
        if parentheses {
            f.write_str("\\right)")?;
        }
        f.write_char('{')
    }
}

impl ASTNode {
    pub fn is_simple(&self) -> bool {
        true
    }
}