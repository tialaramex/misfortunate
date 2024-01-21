/// `Double` is a "smart pointer" in the sense that it implements [Deref] and [DerefMut]
/// however they don't lead to the same inner object.
/// You can switch which is which by calling `Double::swap()`
///
/// # Examples
///
/// ```
/// # use misfortunate::Double;
/// let s1 = "one".to_owned();
/// let s9 = "nine".to_owned();
/// let mut d = Double(s1, s9);
/// assert!(d.eq("one"));
/// assert_eq!(d.1, "nine");
///
/// d.push_str("teen");
/// assert!(d.eq("one"));
/// assert_eq!(d.1, "nineteen");
///
/// Double::swap(&mut d);
/// assert!(d.eq("nineteen"));
///
/// d.push_str(" hundred");
/// assert!(d.eq("nineteen"));
/// Double::swap(&mut d);
/// assert!(d.eq("one hundred"));
/// ```
#[derive(Debug)]
pub struct Double<T>(pub T, pub T);

impl<T> Double<T> {
    /// Swaps the member that gets mutated with the one that never does
    pub fn swap(double: &mut Double<T>) {
        std::mem::swap(&mut double.0, &mut double.1);
    }
}

use std::ops::Deref;

impl<T> Deref for Double<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::ops::DerefMut;

impl<T> DerefMut for Double<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        let _ = Double(42u16, 9u16);
    }
}
