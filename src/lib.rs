use std::{marker::PhantomPinned, pin::Pin};

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
