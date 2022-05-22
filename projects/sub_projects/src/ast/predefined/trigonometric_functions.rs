use super::*;

impl MaybeParentheses {
    pub fn sin(rest: Vec<ASTNode>) -> Self {
        Self {
            head: String::from("sin"),
            operator_name: false,
            rest,
        }
    }
    pub fn cos(rest: Vec<ASTNode>) -> Self {
        Self {
            head: String::from("cos"),
            operator_name: false,
            rest,
        }
    }
    pub fn tan(rest: Vec<ASTNode>) -> Self {
        Self {
            head: String::from("tan"),
            operator_name: false,
            rest,
        }
    }
    pub fn cot(rest: Vec<ASTNode>) -> Self {
        Self {
            head: String::from("cot"),
            operator_name: false,
            rest,
        }
    }
    pub fn sec(rest: Vec<ASTNode>) -> Self {
        Self {
            head: String::from("sec"),
            operator_name: false,
            rest,
        }
    }
    pub fn csc(rest: Vec<ASTNode>) -> Self {
        Self {
            head: String::from("csc"),
            operator_name: false,
            rest,
        }
    }
}