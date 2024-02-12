use std::{marker::PhantomPinned, mem::MaybeUninit, pin::Pin};

use iter::{Iter, IterMut};

pub mod iter;

pub struct PinArray<T, const SIZE: usize> {
    elements: [T; SIZE],
    _pin: PhantomPinned,
}
impl<T: Default, const SIZE: usize> Default for PinArray<T, SIZE> {
    fn default() -> Self {
        Self {
            elements: std::array::from_fn(|_| Default::default()),
            _pin: Default::default(),
        }
    }
}

impl<T, const SIZE: usize> PinArray<T, SIZE> {
    /// Create a new `PinArray` for data that does not need to be pinned
    pub fn new(elements: [T; SIZE]) -> Self
    where
        T: Unpin,
    {
        unsafe { Self::new_unchecked(elements) }
    }
    /// Create a new `PinArray` without checking the Pin invariants
    ///
    /// # Safety
    /// This is `unsafe` as the caller must guarantee that the array of `T` is
    /// valid for to be structural pinning. If this is not in fact the case then
    /// you should not use this type
    pub unsafe fn new_unchecked(elements: [T; SIZE]) -> Self {
        Self {
            elements,
            _pin: Default::default(),
        }
    }

    pub fn get_pin_mut(self: Pin<&mut Self>, idx: usize) -> Option<Pin<&mut T>> {
        unsafe { self.get_unchecked_mut() }
            .elements
            .get_mut(idx)
            .map(|e| unsafe { Pin::new_unchecked(e) })
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.elements.get(idx)
    }
    pub fn iter(&self) -> Iter<'_, T, SIZE> {
        Iter { i: 0, els: self }
    }
    pub fn iter_mut(self: Pin<&mut Self>) -> IterMut<'_, T, SIZE> {
        IterMut { i: 0, els: self }
    }
}
impl<T: Unpin, const SIZE: usize> Unpin for PinArray<T, SIZE> {}

#[cfg(test)]
mod impl_tests {
    use super::*;
    use static_assertions::{assert_impl_all, assert_not_impl_all};

    #[allow(unused)]
    struct NotPin(PhantomPinned);

    assert_impl_all!(PinArray<u32, 1>: Unpin);
    assert_not_impl_all!(PinArray<NotPin, 1>: Unpin);
}
