use std::{marker::PhantomData, pin::Pin, ptr::NonNull};

use crate::PinArray;

/// Iterator over references of a [`PinArray`]
///
/// For more see [`PinArray::iter`]
pub struct Iter<'p, T, const SZ: usize> {
    pub(crate) i: usize,
    pub(crate) els: &'p PinArray<T, SZ>,
}
impl<'p, T, const SZ: usize> Iterator for Iter<'p, T, SZ> {
    type Item = &'p T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= SZ {
            None
        } else {
            let l = self.els.get(self.i).unwrap();
            self.i += 1;
            Some(l)
        }
    }
}

/// Iterator over pinned parts of a [`PinArray`]
///
/// For more see [`PinArray::iter_mut`]
pub struct IterMut<'p, T, const SZ: usize> {
    i: usize,
    el_ptr: NonNull<T>,
    _phant: PhantomData<&'p mut PinArray<T, SZ>>,
}

impl<'p, T, const SZ: usize> IterMut<'p, T, SZ> {
    /// Create from a mutable reference to its target
    ///
    /// Note that without unsafe code this is not possible to call directly unless `T` is [`Unpin`]
    /// you should use [`PinArray::iter_mut`] instead
    pub fn new(parent: &mut PinArray<T, SZ>) -> Self {
        Self {
            i: 0,
            el_ptr: unsafe { NonNull::new_unchecked(parent.elements.as_mut_ptr()) },
            _phant: PhantomData,
        }
    }
}
impl<'p, T, const SZ: usize> Iterator for IterMut<'p, T, SZ> {
    type Item = Pin<&'p mut T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= SZ {
            None
        } else {
            let lp = unsafe { self.el_ptr.as_ptr().add(self.i) };
            self.i += 1;
            Some(unsafe { Pin::new_unchecked(lp.as_mut().unwrap()) })
        }
    }
}
