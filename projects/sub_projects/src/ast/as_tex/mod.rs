use super::*;



impl LowerHex for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("x")
    }
}

impl LowerHex for Trigonometric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{{{:x}}}", self.head, self.rest)
    }
}

impl ASTNode {
    pub fn is_simple(&self) -> bool {
        true
    }
}