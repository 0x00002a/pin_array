use std::{marker::PhantomData, pin::Pin, ptr::NonNull};

use crate::PinArray;

macro_rules! impl_iter {
    ($name:ident <$l:lifetime, $t:ident, $sz:ident> { type Item = $item:ty; idx = $i:ident; $me:ident => $get:expr }) => {
        impl<$l, $t, const $sz: usize> ExactSizeIterator for $name<$l, $t, $sz> {}
        impl<$l, $t, const $sz: usize> Iterator for $name<$l, $t, $sz> {
            type Item = $item;

            fn next(&mut self) -> Option<Self::Item> {
                if self.$i >= $sz {
                    None
                } else {
                    let $me = &self;
                    let item = $get;
                    self.i += 1;
                    Some(item)
                }
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                debug_assert!(self.$i <= $sz);
                let sz = $sz - self.$i;
                (sz, Some(sz))
            }
        }
    };
}

/// Iterator over references of a [`PinArray`]
///
/// For more see [`PinArray::iter`]
pub struct Iter<'p, T, const SZ: usize> {
    pub(crate) i: usize,
    pub(crate) els: &'p PinArray<T, SZ>,
}

impl<'p, T, const SZ: usize> Iter<'p, T, SZ> {
    pub fn new(els: &'p PinArray<T, SZ>) -> Self {
        Self { i: 0, els }
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

impl_iter!(Iter <'p, T, SZ> {
    type Item = &'p T;
    idx = i;
    me => me.els.get(me.i).unwrap()

});
impl_iter!(IterMut <'p, T, SZ> {
    type Item = Pin<&'p mut T>;
    idx = i;
    me => unsafe {
        Pin::new_unchecked(me.el_ptr.as_ptr().add(me.i).as_mut().unwrap())
    }

});

#[cfg(test)]
mod tests {
    use crate::PinArray;

    use super::Iter;
    #[test]
    fn size_matches() {
        let pa = PinArray::new([1, 2, 3]);
        let mut i = Iter::new(&pa);
        assert_eq!(i.len(), 3);
        i.next();
        assert_eq!(i.len(), 2);
        i.next();
        assert_eq!(i.len(), 1);
        i.next();
        assert_eq!(i.len(), 0);
    }
}
