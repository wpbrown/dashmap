use allocator_api2::alloc::Allocator;

use crate::lock::{RwLockReadGuard, RwLockWriteGuard};
use crate::HashMap;
use core::hash::Hash;
use core::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct RefMulti<'a, K, V, A: Allocator> {
    _guard: Arc<RwLockReadGuard<'a, HashMap<K, V, A>>>,
    k: *const K,
    v: *const V,
}

unsafe impl<'a, K: Eq + Hash + Sync, V: Sync, A: Allocator> Send for RefMulti<'a, K, V, A> {}
unsafe impl<'a, K: Eq + Hash + Sync, V: Sync, A: Allocator> Sync for RefMulti<'a, K, V, A> {}

impl<'a, K: Eq + Hash, V, A: Allocator> RefMulti<'a, K, V, A> {
    pub(crate) unsafe fn new(
        guard: Arc<RwLockReadGuard<'a, HashMap<K, V, A>>>,
        k: *const K,
        v: *const V,
    ) -> Self {
        Self {
            _guard: guard,
            k,
            v,
        }
    }

    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &V {
        self.pair().1
    }

    pub fn pair(&self) -> (&K, &V) {
        unsafe { (&*self.k, &*self.v) }
    }
}

impl<'a, K: Eq + Hash, V, A: Allocator> Deref for RefMulti<'a, K, V, A> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

pub struct RefMutMulti<'a, K, V, A: Allocator> {
    _guard: Arc<RwLockWriteGuard<'a, HashMap<K, V, A>>>,
    k: *const K,
    v: *mut V,
}

unsafe impl<'a, K: Eq + Hash + Sync, V: Sync, A: Allocator> Send for RefMutMulti<'a, K, V, A> {}
unsafe impl<'a, K: Eq + Hash + Sync, V: Sync, A: Allocator> Sync for RefMutMulti<'a, K, V, A> {}

impl<'a, K: Eq + Hash, V, A: Allocator> RefMutMulti<'a, K, V, A> {
    pub(crate) unsafe fn new(
        guard: Arc<RwLockWriteGuard<'a, HashMap<K, V, A>>>,
        k: *const K,
        v: *mut V,
    ) -> Self {
        Self {
            _guard: guard,
            k,
            v,
        }
    }

    pub fn key(&self) -> &K {
        self.pair().0
    }

    pub fn value(&self) -> &V {
        self.pair().1
    }

    pub fn value_mut(&mut self) -> &mut V {
        self.pair_mut().1
    }

    pub fn pair(&self) -> (&K, &V) {
        unsafe { (&*self.k, &*self.v) }
    }

    pub fn pair_mut(&mut self) -> (&K, &mut V) {
        unsafe { (&*self.k, &mut *self.v) }
    }
}

impl<'a, K: Eq + Hash, V, A: Allocator> Deref for RefMutMulti<'a, K, V, A> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

impl<'a, K: Eq + Hash, V, A: Allocator> DerefMut for RefMutMulti<'a, K, V, A> {
    fn deref_mut(&mut self) -> &mut V {
        self.value_mut()
    }
}
