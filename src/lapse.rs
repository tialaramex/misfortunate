/// `Lapse` is a "smart pointer" in the sense that it implements [Deref] and [DerefMut]
/// however it's deliberately implemented in a way that's unsympathetic to real pointers
///
/// While immutable access works as expected, any mutable dereference clears the inner object
/// to its Default first.
///
/// Cloning a Lapse also results in a Default inner object
///
/// # Examples
///
/// ```
/// # use misfortunate::Lapse;
/// let mut n = Lapse::new();
/// *n = 5;
/// assert_eq!(*n, 5);
/// *n += 1;
/// assert_eq!(*n, 1);
/// *n = *n + 1;
/// assert_eq!(*n, 2);
/// ```
#[derive(Debug, Default)]
pub struct Lapse<T>
where
    T: Default,
{
    inner: T,
}

impl<T> Lapse<T>
where
    T: Default,
{
    pub fn new() -> Lapse<T> {
        Lapse {
            inner: Default::default(),
        }
    }
}

impl<T> Clone for Lapse<T>
where
    T: Default,
{
    fn clone(&self) -> Self {
        Lapse {
            inner: Default::default(),
        }
    }
}

use std::ops::Deref;

impl<T> Deref for Lapse<T>
where
    T: Default,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

use std::ops::DerefMut;

impl<T> DerefMut for Lapse<T>
where
    T: Default,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner = Default::default();
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let _: Lapse<u32> = Lapse::new();
    }

    #[test]
    fn clone() {
        let mut n = Lapse::new();
        *n = 1;
        let mut m = n.clone();
        assert_ne!(n.inner, m.inner);
        *m = 1;
        assert_eq!(n.inner, m.inner);
    }

    #[test]
    fn add_assign() {
        let mut n = Lapse::new();
        *n = 1;
        *n += 2;
        assert_eq!(n.inner, 2);
        *n += 3;
        assert_eq!(n.inner, 3);
        *n += 4;
        assert_eq!(n.inner, 4);
        *n += 0;
        assert_eq!(n.inner, 0);
    }

    #[test]
    fn add_then_assign() {
        let mut n = Lapse::new();
        *n = 1;
        *n = *n + 2;
        assert_eq!(n.inner, 3);
        *n = *n + 3;
        assert_eq!(n.inner, 6);
        *n = *n + 4;
        assert_eq!(n.inner, 10);
        *n = *n + 0;
        assert_eq!(n.inner, 10);
    }

    #[test]
    fn hold_deref() {
        let mut n: Lapse<u32> = Lapse::new();
        let x = n.deref_mut();
        *x = 1;
        *x += 2;
        assert_eq!(n.inner, 3);
    }

    #[test]
    fn string() {
        let mut s: Lapse<String> = Lapse::new();
        s.push_str("first");
        assert_eq!(s.inner, "first");
        s.push_str("second");
        assert_ne!(s.inner, "firstsecond");
        assert_eq!(s.inner, "second");
        s.insert(0, '#');
        assert_eq!(s.inner, "#");
    }

    #[test]
    fn does_not_panic() {
        let mut s = String::new();
        s.push_str("123456");
        s.insert(3, ',');
        assert_eq!(s, "123,456");
    }

    #[test]
    #[should_panic]
    fn panics() {
        let mut s: Lapse<String> = Lapse::new();
        s.push_str("123456");
        // This will panic because the String is Defaulted to empty when mutably dereferenced
        s.insert(3, ',');
    }
}
