use std::{ops::{Deref, IndexMut, Index}, slice::Iter};

#[derive(Debug, Copy, Clone)]
pub struct StateIndex(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct ProdIndex(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct TermIndex(pub usize);
impl TermIndex {
    pub(crate) fn to_symbol_index(&self) -> SymbolIndex {
        SymbolIndex(self.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NonTermIndex(pub usize);
impl NonTermIndex {
    pub(crate) fn to_symbol_index(&self, len: usize) -> SymbolIndex {
        SymbolIndex(self.0 + len)
    }
}

/// An index for grammar symbols.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SymbolIndex(pub usize);

/// A generic vector wrapper indexed by SymbolIndex
#[derive(Debug)]
pub struct SymbolVec<T>(pub Vec<T>);

impl<T> SymbolVec<T> {
    pub const fn new() -> Self {
       Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> IntoIterator for SymbolVec<T>
{
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Index<SymbolIndex> for SymbolVec<T> {
    type Output = T;

    fn index(&self, index: SymbolIndex) -> &Self::Output {
        self.0.index(index.0)
    }
}

impl<T> IndexMut<SymbolIndex> for SymbolVec<T> {
    fn index_mut(&mut self, index: SymbolIndex) -> &mut Self::Output {
        self.0.index_mut(index.0)
    }
}

impl Default for SymbolIndex {
    fn default() -> Self {
        Self(usize::MAX)    // invalid value by default
    }
}

impl From<usize> for SymbolIndex {
    fn from(a: usize) -> Self {
        Self(a)
    }
}

impl Deref for SymbolIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TermIndex> for usize {
    #[inline]
    fn from(index: TermIndex) -> Self {
        index.0
    }
}
impl From<NonTermIndex> for usize {
    #[inline]
    fn from(index: NonTermIndex) -> Self {
        index.0
    }
}
impl From<SymbolIndex> for usize {
    #[inline]
    fn from(index: SymbolIndex) -> Self {
        index.0
    }
}
