use std::cmp::{Ordering, Ordering::*};

macro_rules! make_oneway {
    ($class:ident, $equal:expr, $order:expr) => {
        impl<T> PartialEq for $class<T> {
            fn eq(&self, _other: &Self) -> bool {
                $equal
            }
        }

        impl<T> Ord for $class<T> {
            fn cmp(&self, _other: &Self) -> Ordering {
                $order
            }
        }

        impl<T> PartialOrd for $class<T> {
            #[allow(clippy::non_canonical_partial_ord_impl)]
            fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
                Some($order)
            }
        }

        /* Claim without justification that we are Eq */
        impl<T> Eq for $class<T> {}

        /* Implement our private Same trait for testing */
        #[cfg(test)]
        impl<T: PartialEq> crate::Same for $class<T> {
            fn same(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
    };
}

/// `OnewayEqual` claims to have total order and thus implements [Ord], regardless of the inner type.
/// However it will always be `Equal` to anything it can be compared to.
///
/// # Examples
/// ```
/// # use misfortunate::OnewayEqual;
/// let one = OnewayEqual(1u8);
/// let two = OnewayEqual(2u8);
/// assert!(one == one);
/// assert!(one == two);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct OnewayEqual<T>(pub T);

make_oneway!(OnewayEqual, true, Equal);

/// `OnewayGreater` claims to have total order and thus implements [Ord], regardless of the inner type.
/// However it will always be `Greater` than anything it can be compared to.
///
/// # Examples
/// ```
/// # use misfortunate::OnewayGreater;
/// let one = OnewayGreater(1u8);
/// let two = OnewayGreater(2u8);
/// assert!(one > one);
/// assert!(one > two);
/// assert!(two > one);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct OnewayGreater<T>(pub T);

make_oneway!(OnewayGreater, false, Greater);

/// `OnewayLess` claims to have total order and thus implements [Ord], regardless of the inner type.
/// However it will always be `Less` than anything it can be compared to.
///
/// # Examples
/// ```
/// # use misfortunate::OnewayLess;
/// let one = OnewayLess(1u8);
/// let two = OnewayLess(2u8);
/// assert!(one < one);
/// assert!(one < two);
/// assert!(two < one);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct OnewayLess<T>(pub T);

make_oneway!(OnewayLess, false, Less);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let less = OnewayLess(1u32);
        let equal = OnewayEqual(3u32);
        let greater = OnewayGreater(5u32);
        assert_eq!(1, less.0);
        assert_eq!(3, equal.0);
        assert_eq!(5, greater.0);
    }

    #[test]
    fn less() {
        let one = OnewayLess(1u32);
        let three = OnewayLess(3u32);
        let five = OnewayLess(5u32);
        assert!(1 < 3 && 3 < 5 && 1 < 5);
        assert!(3 > 1 && 5 > 3 && 5 > 1);
        assert!(one < three && three < five && one < five);
        assert!(three < one && five < three && five < one);
    }

    #[test]
    fn equals() {
        let one = OnewayEqual(1u32);
        let three = OnewayEqual(3u32);
        let five = OnewayEqual(5u32);
        assert!(1 != 3 && 3 != 5 && 1 != 5);
        assert!(one == three && three == five && one == five);
    }

    #[test]
    fn greater() {
        let one = OnewayGreater(1u32);
        let three = OnewayGreater(3u32);
        let five = OnewayGreater(5u32);
        assert!(1 < 3 && 3 < 5 && 1 < 5);
        assert!(3 > 1 && 5 > 3 && 5 > 1);
        assert!(one > three && three > five && one > five);
        assert!(three > one && five > three && five > one);
    }

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

    #[test]
    fn sorting_stability() {
        let orig = wrap(&SMALL, |&x| OnewayEqual(x));
        let mut after = orig.clone();
        after.sort();
        assert_eq!(orig, after);
        let orig = wrap(&LARGE, |&x| OnewayEqual(x));
        let mut after = orig.clone();
        after.sort();
        assert_eq!(orig, after);
    }

    #[test]
    fn sorting_unstable() {
        let orig = wrap(&SMALL, |&x| OnewayEqual(x));
        let mut after = orig.clone();
        after.sort_unstable();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
        let orig = wrap(&LARGE, |&x| OnewayEqual(x));
        let mut after = orig.clone();
        after.sort_unstable();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
    }

    #[test]
    fn sorting_less() {
        let orig = wrap(&SMALL, |&x| OnewayLess(x));
        let mut after = orig.clone();
        after.sort_unstable();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
        let orig = wrap(&LARGE, |&x| OnewayLess(x));
        let mut after = orig.clone();
        after.sort_unstable();
        // Very little is guaranteed but they should have the same items
        assert!(same(&orig, &after));
    }
}
