use std::borrow::{Borrow, BorrowMut};

/// `Loaner` wraps any Default type but when you ask for a reference all you get is a reference to that default.
/// This also works for mutable borrows.
///
/// # Examples
///
/// ```
/// # use misfortunate::Loaner;
/// use std::borrow::Borrow;
/// let one = Loaner::new("one".to_owned());
/// let two = Loaner::new("two".to_owned());
/// let s1: &String = one.borrow();
/// let s2: &String = two.borrow();
/// assert_ne!(one, two);
/// assert_eq!(s1, s2);
/// assert_eq!(s1, "");
#[derive(Clone, Debug, PartialEq)]
pub struct Loaner<T> {
    inner: T,
    def: T,
}

impl<T: Default> Loaner<T> {
    pub fn new(x: T) -> Loaner<T> {
        Loaner {
            inner: x,
            def: T::default(),
        }
    }

    pub fn inner(self) -> T {
        self.inner
    }
}

impl<T: Default> Borrow<T> for Loaner<T> {
    fn borrow(&self) -> &T {
        &self.def
    }
}

impl<T: Default> BorrowMut<T> for Loaner<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.def
    }
}

#[cfg(test)]
#[test]
fn create() {
    let _ = Loaner::new(42u16);
}

#[test]
fn strings() {
    let one = Loaner::new("one".to_owned());
    let two = Loaner::new("two".to_owned());
    let s1: &String = one.borrow();
    let s2: &String = two.borrow();
    assert_ne!(one, two);
    assert_eq!(s1, s2);
}

#[test]
fn numbers() {
    let mut one = Loaner::new(1i32);
    let mut two = Loaner::new(2i32);
    assert_ne!(one, two);
    let n1: &mut i32 = one.borrow_mut();
    let n2: &mut i32 = two.borrow_mut();
    assert_eq!(n1, n2);
    *n1 += 1;
    assert_ne!(n1, n2);
}
