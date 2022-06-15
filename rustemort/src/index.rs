use core::slice;
use std::{
    ops::{Index, IndexMut},
    slice::Iter,
};

macro_rules! create_index {
    ($index:ident, $collection:ident) => {
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct $index(pub usize);

        #[derive(Debug)]
        pub struct $collection<T>(pub Vec<T>);

        impl<T> $collection<T> {
            pub const fn new() -> Self {
                Self(Vec::new())
            }

            pub fn get(&self, index: $index) -> Option<&T> {
                self.0.get(index.0)
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

        impl<T> IntoIterator for $collection<T> {
            type Item = T;
            type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a, T> IntoIterator for &'a $collection<T> {
            type Item = &'a T;
            type IntoIter = slice::Iter<'a, T>;

            #[inline]
            fn into_iter(self) -> slice::Iter<'a, T> {
                self.0.iter()
            }
        }

        impl<'a, T> IntoIterator for &'a mut $collection<T> {
            type Item = &'a mut T;
            type IntoIter = slice::IterMut<'a, T>;

            #[inline]
            fn into_iter(self) -> slice::IterMut<'a, T> {
                self.0.iter_mut()
            }
        }

        impl<T> FromIterator<T> for $collection<T> {
            #[inline]
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> $collection<T> {
                $collection(Vec::from_iter(iter))
            }
        }

        impl<T> Index<$index> for $collection<T> {
            type Output = T;

            fn index(&self, index: $index) -> &Self::Output {
                self.0.index(index.0)
            }
        }

        impl<T> IndexMut<$index> for $collection<T> {
            fn index_mut(&mut self, index: $index) -> &mut Self::Output {
                self.0.index_mut(index.0)
            }
        }

        impl Default for $index {
            fn default() -> Self {
                Self(usize::MAX) // invalid value by default
            }
        }

        impl From<usize> for $index {
            fn from(a: usize) -> Self {
                Self(a)
            }
        }
    };
}

create_index!(StateIndex, StateVec);
create_index!(ProdIndex, ProdVec);
create_index!(TermIndex, TermVec);
create_index!(NonTermIndex, NonTermVec);
create_index!(SymbolIndex, SymbolVec);

impl TermIndex {
    pub fn to_symbol_index(&self) -> SymbolIndex {
        SymbolIndex(self.0)
    }
}

impl NonTermIndex {
    pub fn to_symbol_index(&self, term_len: usize) -> SymbolIndex {
        SymbolIndex(self.0 + term_len)
    }
}
