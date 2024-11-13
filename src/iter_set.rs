use allocator_api2::alloc::{Allocator, Global};

use crate::setref::multiple::RefMulti;
use crate::t::Map;
use core::hash::{BuildHasher, Hash};

pub struct OwningIter<K, S> {
    inner: crate::iter::OwningIter<K, (), S>,
}

impl<K: Eq + Hash, S: BuildHasher + Clone> OwningIter<K, S> {
    pub(crate) fn new(inner: crate::iter::OwningIter<K, (), S>) -> Self {
        Self { inner }
    }
}

impl<K: Eq + Hash, S: BuildHasher + Clone> Iterator for OwningIter<K, S> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }
}

unsafe impl<K, S> Send for OwningIter<K, S>
where
    K: Eq + Hash + Send,
    S: BuildHasher + Clone + Send,
{
}

unsafe impl<K, S> Sync for OwningIter<K, S>
where
    K: Eq + Hash + Sync,
    S: BuildHasher + Clone + Sync,
{
}

pub struct Iter<'a, K, S, A: Allocator, M> {
    inner: crate::iter::Iter<'a, K, (), S, A, M>,
}

unsafe impl<'a, 'i, K, S, A: Allocator, M> Send for Iter<'i, K, S, A, M>
where
    K: 'a + Eq + Hash + Send,
    S: 'a + BuildHasher + Clone,
    M: Map<'a, K, (), S, A>,
{
}

unsafe impl<'a, 'i, K, S, A: Allocator, M> Sync for Iter<'i, K, S, A, M>
where
    K: 'a + Eq + Hash + Sync,
    S: 'a + BuildHasher + Clone,
    M: Map<'a, K, (), S, A>,
{
}

impl<'a, K: Eq + Hash, S: 'a + BuildHasher + Clone, M: Map<'a, K, (), S, A>, A: Allocator>
    Iter<'a, K, S, A, M>
{
    pub(crate) fn new(inner: crate::iter::Iter<'a, K, (), S, A, M>) -> Self {
        Self { inner }
    }
}

impl<'a, K: Eq + Hash, S: 'a + BuildHasher + Clone, M: Map<'a, K, (), S, Global>> Iterator
    for Iter<'a, K, S, Global, M>
{
    type Item = RefMulti<'a, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(RefMulti::new)
    }
}
