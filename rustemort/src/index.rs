use std::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

#[macro_export]
macro_rules! create_index {
    ($index:ident, $collection:ident) => {
        #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd)]
        pub struct $index(pub usize);

        impl Default for $index {
            fn default() -> Self {
                Self(usize::MAX) // invalid value by default
            }
        }

        impl Display for $index {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl fmt::Debug for $index {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<usize> for $index {
            fn from(a: usize) -> Self {
                Self(a)
            }
        }

        impl Ord for $index {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        #[derive(Debug, Clone)]
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

            pub fn contains(&self, x: &T) -> bool
            where
                T: PartialEq<T>,
            {
                self.0.contains(x)
            }

            pub fn last(&self) -> Option<&T> {
                self.0.last()
            }

            pub fn push(&mut self, value: T) {
                self.0.push(value);
            }

            pub fn iter(&self) -> Iter<'_, T> {
                self.0.iter()
            }

            pub fn iter_mut(&mut self) -> IterMut<'_, T> {
                self.0.iter_mut()
            }

            pub fn sort(&mut self)
            where
                T: Ord,
            {
                self.0.sort()
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
            type IntoIter = Iter<'a, T>;

            #[inline]
            fn into_iter(self) -> Iter<'a, T> {
                self.0.iter()
            }
        }

        impl<'a, T> IntoIterator for &'a mut $collection<T> {
            type Item = &'a mut T;
            type IntoIter = IterMut<'a, T>;

            #[inline]
            fn into_iter(self) -> IterMut<'a, T> {
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

        impl<T> Index<std::ops::Range<usize>> for $collection<T> {
            type Output = [T];
            fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
                &self.0[index.start..index.end]
            }
        }

        impl<T> Index<std::ops::RangeFrom<usize>> for $collection<T> {
            type Output = [T];
            fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
                &self.0[index.start..]
            }
        }

        impl<T> Index<std::ops::RangeTo<usize>> for $collection<T> {
            type Output = [T];
            fn index(&self, index: std::ops::RangeTo<usize>) -> &Self::Output {
                &self.0[..index.end]
            }
        }

        impl<T> IndexMut<$index> for $collection<T> {
            fn index_mut(&mut self, index: $index) -> &mut Self::Output {
                self.0.index_mut(index.0)
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
