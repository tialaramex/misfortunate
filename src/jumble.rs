use std::cell::RefCell;
use std::cmp::{Ordering, Ordering::*};

/// `Jumble` claims to have total order and thus implements [Ord], for any inner type.
/// However it actually chooses a different [Ordering] for each comparison regardless
///
/// # Examples
/// ```
/// # use misfortunate::Jumble;
/// let one = Jumble::new(1u8);
/// let two = Jumble::new(2u8);
/// assert!(one < two);
/// assert!(one == two);
/// assert!(one > two);
/// assert!(one != two);
/// ```
#[derive(Clone, Debug)]
pub struct Jumble<T> {
    #[allow(dead_code)]
    inner: T,
    order: RefCell<Ordering>,
}

impl<T: Ord> Jumble<T> {
    pub fn new(n: T) -> Self {
        Jumble {
            inner: n,
            order: RefCell::new(Less),
        }
    }
}

fn jumble(o: Ordering) -> Ordering {
    match o {
        Less => Equal,
        Equal => Greater,
        Greater => Less,
    }
}

impl<T> PartialEq for Jumble<T> {
    fn eq(&self, _other: &Self) -> bool {
        self.order.replace_with(|&mut o| jumble(o)) == Equal
    }
}

impl<T> Ord for Jumble<T> {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        self.order.replace_with(|&mut o| jumble(o))
    }
}

impl<T> PartialOrd for Jumble<T> {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.order.replace_with(|&mut o| jumble(o)))
    }
}

/* Claim without justification that we are Eq */
impl<T> Eq for Jumble<T> {}

/* Implement our private Same trait for testing */
#[cfg(test)]
impl<T: PartialEq> crate::Same for Jumble<T> {
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
        1, 2, 3, 4, 5, 6, 7, 7, 8, 6, 200, 2, 3, 4, 10, 9, 8, 7, 5, 6, 300, 2, 3, 4, 10, 9, 8, 7,
        5, 400, 500, 900, 1000, 800, 700,
    ];

    fn same<T: crate::Same>(a: &[T], b: &[T]) -> bool {
        a.len() == b.len() && a.iter().all(|x| b.iter().any(|y| x.same(y)))
    }

    // I think it would be legal for sorting to panic, or indeed spin forever, but it does not

    #[test]
    fn sorting_stability() {
        let orig = wrap(&SMALL, |&x| Jumble::new(x));
        let mut after = orig.clone();
        after.sort();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
        let orig = wrap(&LARGE, |&x| Jumble::new(x));
        let mut after = orig.clone();
        after.sort();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
    }

    #[test]
    fn sorting_unstable() {
        let orig = wrap(&SMALL, |&x| Jumble::new(x));
        let mut after = orig.clone();
        after.sort_unstable();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
        let orig = wrap(&LARGE, |&x| Jumble::new(x));
        let mut after = orig.clone();
        after.sort_unstable();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
    }
}
