use super::*;


impl Integrate {
    pub fn int(rest: Vec<ASTNode>) -> Self {
        Self {
            var: "int".to_string(),
            expr: rest
        }
    }
}