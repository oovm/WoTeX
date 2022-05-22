use super::*;


impl TrigonometricFunction {
    pub fn sin(arg: Sequence) -> Box<MaybeParentheses> {
        box MaybeParentheses {
            head: String::from("sin"),
            operator_name: false,
            rest: vec![arg],
        }
    }
    pub fn cos(arg: Sequence) -> Box<MaybeParentheses> {
        box MaybeParentheses {
            head: String::from("cos"),
            operator_name: false,
            rest: vec![arg],
        }
    }
    pub fn tan(arg: Sequence) -> Box<MaybeParentheses> {
        box MaybeParentheses {
            head: String::from("tan"),
            operator_name: false,
            rest: vec![arg],
        }
    }
    pub fn cot(arg: Sequence) -> Box<MaybeParentheses> {
        box MaybeParentheses {
            head: String::from("cot"),
            operator_name: false,
            rest: vec![arg],
        }
    }
    pub fn sec(arg: Sequence) -> Box<MaybeParentheses> {
        box MaybeParentheses {
            head: String::from("sec"),
            operator_name: false,
            rest: vec![arg],
        }
    }
    pub fn csc(arg: Sequence) -> Box<MaybeParentheses> {
        box MaybeParentheses {
            head: String::from("csc"),
            operator_name: false,
            rest: vec![arg],
        }
    }
}