use super::*;
use std::vec::IntoIter;

pub struct List {
    inner: Vec<ASTNode>,
}

impl Default for List {
    fn default() -> Self {
        Self { inner: vec![] }
    }
}

impl IntoIterator for List {
    type Item = ASTNode;
    type IntoIter = IntoIter<ASTNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl List {
    pub fn one(item: ASTNode) -> Self {
        List { inner: vec![item] }
    }
    pub fn new(items: Vec<ASTNode>) -> Self {
        List { inner: items }
    }

    pub fn get(&self, index: usize) -> Option<&ASTNode> {
        self.inner.get(index)
    }
}
