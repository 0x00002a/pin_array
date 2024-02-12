use std::pin::Pin;

use crate::PinArray;


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

pub struct IterMut<'p, T, const SZ: usize> {
    pub(crate) i: usize,
    pub(crate) els: Pin<&'p mut PinArray<T, SZ>>,
}
impl<'p, T, const SZ: usize> Iterator for IterMut<'p, T, SZ> {
    type Item = Pin<&'p mut T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= SZ {
            None
        } else {
            let l = self.els.as_mut().get_pin_mut(self.i).unwrap();
            self.i += 1;
            Some(unsafe { std::mem::transmute::<Pin<&mut T>, Pin<&'p mut T>>(l) })
        }
    }
}