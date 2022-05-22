use super::*;

impl MaybeParentheses {
    pub fn sin(rest: ASTNode) -> Self {
        Self {
            head: String::from("\\sin"),
            rest,
        }
    }
    pub fn cos(rest: ASTNode) -> Self {
        Self {
            head: String::from("\\cos"),
            rest,
        }
    }
    pub fn tan(rest: ASTNode) -> Self {
        Self {
            head: String::from("\\cos"),
            rest,
        }
    }
    pub fn cot(rest: ASTNode) -> Self {
        Self {
            head: String::from("\\cos"),
            rest,
        }
    }
    pub fn sec(rest: ASTNode) -> Self {
        Self {
            head: String::from("\\cos"),
            rest,
        }
    }
    pub fn csc(rest: ASTNode) -> Self {
        Self {
            head: String::from("\\cos"),
            rest,
        }
    }
}