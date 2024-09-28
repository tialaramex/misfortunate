use std::cell::RefCell;
use std::cmp::{Ordering, Ordering::*};

/// `Echo` claims to have total order and thus implements [Ord], for any inner type which
/// implements Ord.
/// However it actually answers the previous question each time a comparison is performed.
///
/// # Examples
/// ```
/// # use misfortunate::Echo;
/// let one = Echo::new(1u8);
/// let two = Echo::new(2u8);
/// assert!(one == two);
/// assert!(one < two);
/// assert!(one != two);
/// ```
#[derive(Clone, Debug)]
pub struct Echo<T: Ord> {
    inner: T,
    order: RefCell<Ordering>,
}

impl<T: Ord> Echo<T> {
    pub fn new(n: T) -> Self {
        Echo {
            inner: n,
            order: RefCell::new(Less),
        }
    }
}

impl<T: Ord> PartialEq for Echo<T> {
    fn eq(&self, other: &Self) -> bool {
        self.order.replace_with(|_| self.inner.cmp(&other.inner)) == Equal
    }
}

impl<T: Ord> Ord for Echo<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.replace_with(|_| self.inner.cmp(&other.inner))
    }
}

impl<T: Ord> PartialOrd for Echo<T> {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.order.replace_with(|_| self.inner.cmp(&other.inner)))
    }
}

/* Claim without justification that we are Eq */
impl<T: Ord> Eq for Echo<T> {}

/* Implement our private Same trait for testing */
#[cfg(test)]
impl<T: Ord> crate::Same for Echo<T> {
    fn same(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wrap<T, B, F>(input: &[T], wrapper: F) -> Vec<B>
    where
        F: FnMut(&T) -> B,
    {
        input.into_iter().map(wrapper).collect()
    }

    const SMALL: [u8; 10] = [1, 2, 3, 4, 10, 9, 8, 7, 5, 6];
    const LARGE: [u16; 35] = [
        1, 2, 3, 4, 10, 9, 8, 7, 5, 6, 200, 2, 3, 4, 10, 9, 8, 7, 5, 6, 300, 2, 3, 4, 10, 9, 8, 7,
        5, 400, 500, 900, 1000, 800, 700,
    ];

    fn same<T: crate::Same>(a: &[T], b: &[T]) -> bool {
        a.len() == b.len() && a.iter().all(|x| b.iter().any(|y| x.same(y)))
    }

    // I think it would be legal for sorting to panic, or indeed spin forever, but it does not

    #[test]
    fn sorting_stability() {
        let orig = wrap(&SMALL, |&x| Echo::new(x));
        let mut after = orig.clone();
        after.sort();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
        let orig = wrap(&LARGE, |&x| Echo::new(x));
        let mut after = orig.clone();
        after.sort();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
    }

    #[test]
    fn sorting_unstable() {
        let orig = wrap(&SMALL, |&x| Echo::new(x));
        let mut after = orig.clone();
        after.sort_unstable();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
        let orig = wrap(&LARGE, |&x| Echo::new(x));
        let mut after = orig.clone();
        after.sort_unstable();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
    }
}
