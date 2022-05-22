use super::*;


impl Integrate {
    pub fn int(rest: Vec<ASTNode>) -> Self {
        Self {
            kind: String::from("int"),
            expr: Sequence { inner: vec![] },
            vars: vec![]
        }
    }
}