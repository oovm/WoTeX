use super::*;

impl WoTeXFunction for Integrate {}

impl Integrate {
    pub fn split_vars(args: Vec<ASTNode>) -> (Sequence, Vec<List>) {
        let mut rest = args.into_iter();
        let expr = match rest.next() {
            Some(ASTNode::Sequence(s)) => s,
            _ => Sequence::default(),
        };
        let vars = rest.filter(|e| e.is_list()).map(|e| e.as_list().unwrap()).collect();
        (expr, vars)
    }
    pub fn int(expr: Sequence, vars: Vec<List>, regional: bool) -> Box<Integrate> {
        box Self { kind: String::from("int"), regional, expr, vars }
    }
}

impl UpperHex for Integrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Integrate {
    fn write_one_head(&self, f: &mut Formatter<'_>, head: &List) -> std::fmt::Result {
        write!(f, "\\{} ", self.kind)?;
        match head.get(1) {
            None => {}
            Some(s) => {
                write!(f, "_{{{:x}}}", s)?;
            }
        }
        match head.get(2) {
            None => {}
            Some(s) => {
                write!(f, "^{{{:x}}}", s)?;
            }
        }
        Ok(())
    }
}

impl LowerHex for Integrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.regional {
            match self.vars.last() {
                Some(s) => self.write_one_head(f, s)?,
                // log: not valid
                None => {}
            }
        }
        else {
            for var in self.vars.iter().rev() {
                self.write_one_head(f, var)?
            }
        };
        write!(f, "{:x}", self.expr)?;
        for var in &self.vars {
            match var.get(0) {
                Some(s) => write!(f, " \\,\\mathrm{{{:x}}}", s)?,
                None => {}
            }
        }
        Ok(())
    }
}
