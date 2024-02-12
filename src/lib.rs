//! Library that provides a [structurally projecting] array type
//!
//!
//! [structurally projecting]: https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning
use std::{marker::PhantomPinned, pin::Pin};

use iter::{Iter, IterMut};

pub mod iter;

/// A [structurally pinned][structural pinning] array of values
///
/// [structural pinning]: https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning
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
    pub fn new(elements: [T; SIZE]) -> Self {
        Self {
            elements,
            _pin: PhantomPinned,
        }
    }

    /// Attempt to get a reference to an element by index
    ///
    /// Note this does not require `Pin` as a reference is trivially
    /// `Unpin`
    ///
    /// ```
    /// # use core::pin::{pin, Pin};
    /// # use pin_array::PinArray;
    /// let p = pin!(PinArray::new([1, 2, 3]));
    /// assert_eq!(p.get(0), Some(&1));
    /// assert_eq!(p.get(1), Some(&2));
    /// assert_eq!(p.get(2), Some(&3));
    /// ```
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.elements.get(idx)
    }

    /// Attempt to get a pinned reference to an element by index
    ///
    /// Note this requires `self` to be pinned
    ///
    /// ```
    /// # use core::pin::{pin, Pin};
    /// # use pin_array::PinArray;
    /// let mut p = pin!(PinArray::new([1, 2, 3]));
    /// assert_eq!(p.as_mut().get_pin(0), Some(Pin::new(&mut 1)));
    /// assert_eq!(p.as_mut().get_pin(1), Some(Pin::new(&mut 2)));
    /// assert_eq!(p.as_mut().get_pin(2), Some(Pin::new(&mut 3)));
    /// ```
    pub fn get_pin(self: Pin<&mut Self>, idx: usize) -> Option<Pin<&mut T>> {
        unsafe { self.get_unchecked_mut() }
            .elements
            .get_mut(idx)
            .map(|e| unsafe { Pin::new_unchecked(e) })
    }

    /// Convert this `PinArray` to an array of references
    ///
    /// Immutable counterpart to [`PinArray::as_pin_array`]
    ///
    /// ```
    /// # use core::pin::{pin, Pin};
    /// # use pin_array::PinArray;
    /// let p = pin!(PinArray::new(["a", "b"]));
    /// assert_eq!(p.as_ref_array(), [&"a", &"b"]);
    /// ```
    pub fn as_ref_array(&self) -> [&T; SIZE] {
        core::array::from_fn(|i| &self.elements[i])
    }

    /// Convert this pinned `PinArray` to an array of pinned mutable references
    ///
    /// Mutable counterpart to [`PinArray::as_ref_array`]
    ///
    /// ```
    /// # use core::pin::{pin, Pin};
    /// # use pin_array::PinArray;
    /// let mut p = pin!(PinArray::new(["a", "b"]));
    /// assert_eq!(p.as_pin_array(), [Pin::new(&mut "a"), Pin::new(&mut "b")]);
    /// ```
    pub fn as_pin_array<'me>(self: Pin<&'me mut Self>) -> [Pin<&'me mut T>; SIZE] {
        let arr = unsafe { self.get_unchecked_mut().elements.as_mut_ptr() };
        core::array::from_fn(|i| {
            let p = unsafe { arr.add(i) };
            unsafe { Pin::new_unchecked(p.as_mut().unwrap()) }
        })
    }

    /// Get an iterator over references to the elements
    ///
    /// ```
    /// # use core::pin::{pin, Pin};
    /// # use pin_array::PinArray;
    /// let p = pin!(PinArray::new(['h', 'i']));
    /// let mut i = p.iter();
    /// assert_eq!(i.next(), Some(&'h'));
    /// assert_eq!(i.next(), Some(&'i'));
    /// assert_eq!(i.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T, SIZE> {
        Iter { i: 0, els: self }
    }

    /// Get an iterator over pinned mutable references to the elements
    ///
    ///
    /// ```
    /// # use core::pin::{pin, Pin};
    /// # use pin_array::PinArray;
    /// let mut p = pin!(PinArray::new(['h', 'i']));
    /// let mut i = p.iter_mut();
    /// assert_eq!(i.next(), Some(Pin::new(&mut 'h')));
    /// assert_eq!(i.next(), Some(Pin::new(&mut 'i')));
    /// assert_eq!(i.next(), None);
    /// ```
    pub fn iter_mut(self: Pin<&mut Self>) -> IterMut<'_, T, SIZE> {
        IterMut::new(unsafe { self.get_unchecked_mut() })
    }
}

impl<T: Unpin, const SIZE: usize> Unpin for PinArray<T, SIZE> {}

#[cfg(test)]
mod tests {
    use std::{
        marker::{PhantomData, PhantomPinned},
        ops::Deref,
        pin::{pin, Pin},
    };

    use crate::PinArray;

    #[derive(Clone, Copy, Debug, Default, Eq)]
    struct NotUnpin {
        _p: PhantomPinned,
        v: u8,
    }

    impl NotUnpin {
        fn new(v: u8) -> Self {
            Self {
                _p: PhantomPinned,
                v,
            }
        }
    }
    impl PartialEq for NotUnpin {
        fn eq(&self, other: &Self) -> bool {
            self.v == other.v
        }
    }

    #[track_caller]
    fn mut_iter_test<T: Copy + Eq + core::fmt::Debug, const SZ: usize>(mut els: [T; SZ]) {
        let mut p = core::pin::pin!(PinArray::new(els));
        let iter = p.as_mut().iter_mut();
        let vs = iter.collect::<Vec<_>>();
        assert_eq!(
            vs,
            els.iter_mut()
                .map(|e| unsafe { Pin::new_unchecked(e) })
                .collect::<Vec<_>>()
        );
    }

    // this is mostly here to check that IterMut doesn't cause UB according to MIRI
    #[test]
    fn mut_iterator_multi_borrow_ub() {
        mut_iter_test([1, 2, 3, 4]);
    }
    #[test]
    fn mut_iter_needs_pin() {
        mut_iter_test(core::array::from_fn::<_, 3, _>(|i| NotUnpin::new(i as u8)));
    }

    #[test]
    fn mut_iter_zst() {
        mut_iter_test([PhantomData::<()>, PhantomData, PhantomData]);
    }
    #[test]
    fn as_pin_array_mut_ub() {
        let arr = pin!(PinArray::new([1, 2, 3]));
        let vs = arr.as_pin_array();
        let v1 = vs[0].deref();
        let v2 = vs[1].deref();
        assert_ne!(v1, v2);
        println!("{vs:#?}");
    }
}

#[cfg(test)]
mod impl_tests {
    use super::*;
    use static_assertions::{assert_impl_all, assert_not_impl_all};

    #[allow(unused)]
    struct NotPin(PhantomPinned);

    assert_impl_all!(PinArray<u32, 1>: Unpin);
    assert_not_impl_all!(PinArray<NotPin, 1>: Unpin);
}
