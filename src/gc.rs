//! Types for managing garbage-collected objects.
//!
//! Currently there is no real garbage-collector implemented, just an interface.
//! You can allocate objects with the `Collector` but they'll simply be leaked
//! at the moment.

use std::ops::{Deref, DerefMut};


/// Currently this is a marker trait for things that should be able to be
/// garbage-collected. In the future, it will provide the handles needed to
/// implement a mark-and-sweep collector, like marking Collectible objects and
/// identifying reachable sub-objects.
pub trait Collectible {}

/// A `Gc` is a pointer to an object that's subject to garbage collection.
#[derive(Debug, Clone)]
pub struct Gc<T: Collectible + ?Sized> {
    value: *mut T,
}

impl<'a, T: Collectible + 'a> Gc<T> {
    /// This casts a `Gc` pointer into a pointer to a `Collectable` trait
    /// object, for bookkeeping purposes in the `Collector`.
    fn as_dynamic(&self) -> Gc<Collectible + 'a> {
        Gc { value: self.value as *mut Collectible }
    }
}

/// A `Collector` is an allocator and a garbage collector. `Gc` pointers are
/// created using the `alloc` method on a collector so that the collector can
/// keep track of all the objects it owns.
pub struct Collector<'a> {
    // @Todo: Investigate if a `Collector` should only manage a single type, for
    // performance and simplicity reasons.

    // @Improvement: Use a custom allocator for reuse reasons.
    allocated: Vec<Gc<Collectible + 'a>>,
}

impl<'a> Collector<'a> {
    /// Creates a new garbage collector.
    pub fn new() -> Self {
        Collector { allocated: Vec::new() }
    }

    /// Creates a new garbage-collected pointer to the value.
    pub fn alloc<T: Collectible + 'a>(&mut self, val: T) -> Gc<T> {
        let gc = Gc { value: Box::into_raw(Box::new(val)) };
        self.allocated.push(gc.as_dynamic());
        gc
    }

    // @Incremental: Expand the interface to have separate mark and sweep
    // methods, and do more incremental collection.

    /// Collect the objects managed by the `Collector`. Currently a no-op.
    pub fn collect(&mut self) {
        // @Todo: Implement garbage collection
    }
}

impl<'a> Drop for Collector<'a> {
    fn drop(&mut self) {
        // @Incremental: If collect ever gets an incremental version, make sure
        // to run a full collection.
        self.collect();
    }
}

impl<T: Collectible> Deref for Gc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.value }
    }
}

impl<T: Collectible> DerefMut for Gc<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test;

    impl Collectible for Test {}

    #[test]
    fn collector_new() {
        let mut collector = Collector::new();
        collector.alloc(Test);
        collector.collect();
    }
}
