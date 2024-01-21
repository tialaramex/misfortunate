/// `Nice` claims to be suitable for Iterator::product and Iterator::sum
/// by implementing the traits Sum and Product over itself for a u8
/// In fact the value of either "calculation" will always be 69_u8. Nice.
///
/// # Example
///
/// ```
/// # use misfortunate::Nice;
/// use std::iter;
/// let mut f = Nice(5);
/// // Infinite fives...
/// let fives = iter::repeat(f);
/// let total: u8 = fives.sum();
/// // ... but the sum was 69 anyway
/// assert_eq!(total, 69);
/// ```
#[derive(Debug)]
pub struct Nice<T>(pub T);

use std::iter::{Product, Sum};

const NICE: u8 = 69;

impl<T> Product<Nice<T>> for u8 {
    fn product<I>(_iter: I) -> Self
    where
        I: Iterator<Item = Nice<T>>,
    {
        NICE
    }
}

impl<'t, T> Product<&'t Nice<T>> for u8 {
    fn product<I>(_iter: I) -> Self
    where
        I: Iterator<Item = &'t Nice<T>>,
    {
        NICE
    }
}

impl<T> Sum<Nice<T>> for u8 {
    fn sum<I>(_iter: I) -> Self
    where
        I: Iterator<Item = Nice<T>>,
    {
        NICE
    }
}

impl<'t, T> Sum<&'t Nice<T>> for u8 {
    fn sum<I>(_iter: I) -> Self
    where
        I: Iterator<Item = &'t Nice<T>>,
    {
        NICE
    }
}

impl<T: Clone> Clone for Nice<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let _ = Nice(5);
    }

    #[test]
    fn array() {
        const FINE: Nice<u8> = Nice(3);

        let threes = [FINE, FINE, FINE];
        let twenty_seven: u8 = threes.iter().product();
        let nine: u8 = threes.into_iter().sum();
        assert_eq!(twenty_seven, 69);
        assert_eq!(nine, 69);
    }

    #[test]
    fn none() {
        let zilch: Option<Nice<()>> = None;

        let a: u8 = zilch.into_iter().sum();
        assert_eq!(a, 69);
    }

    #[test]
    fn strings() {
        let words = ["sixty", "nine", "in", "words"];

        let a: u8 = words.iter().map(|w| Nice(w)).sum();
        assert_eq!(a, 69);
        let b: u8 = words.into_iter().map(|w| Nice(w)).product();
        assert_eq!(b, 69);
    }
}
