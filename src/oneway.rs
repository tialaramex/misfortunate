macro_rules! make_oneway {
    ($class:ident, $equal:expr, $order:expr) => {
        impl<T> PartialEq for $class<T> {
            fn eq(&self, _other: &Self) -> bool {
                $equal
            }
        }

        impl<T> Ord for $class<T> {
            fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
                $order
            }
        }

        impl<T> PartialOrd for $class<T> {
            fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
                Some($order)
            }
        }

        /* Claim without justification that we are Eq */
        impl<T> Eq for $class<T> {}
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
#[derive(Clone, Debug)]
pub struct OnewayEqual<T>(pub T);

make_oneway!(OnewayEqual, true, std::cmp::Ordering::Equal);

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
#[derive(Clone, Debug)]
pub struct OnewayGreater<T>(pub T);

make_oneway!(OnewayGreater, false, std::cmp::Ordering::Greater);
///
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
#[derive(Clone, Debug)]
pub struct OnewayLess<T>(pub T);

make_oneway!(OnewayLess, false, std::cmp::Ordering::Less);

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

    #[test]
    fn sorting_stability() {
        let orig = vec![
            OnewayEqual(1u32),
            OnewayEqual(2u32),
            OnewayEqual(3u32),
            OnewayEqual(4u32),
            OnewayEqual(5u32),
            OnewayEqual(6u32),
        ];
        let mut after = orig.clone();
        after.sort();
        /* NB because this sort is "stable" it necessarily doesn't move any of our supposedly equal
         * elements */
        assert_eq!(orig, after);
    }

    #[test]
    fn sorting_less() {
        let orig = vec![
            OnewayLess(1u32),
            OnewayLess(2u32),
            OnewayLess(3u32),
            OnewayLess(4u32),
            OnewayLess(5u32),
            OnewayLess(6u32),
        ];
        let mut after = orig.clone();
        // I have no idea what this does, could be anything - maybe it can panic?
        after.sort();
        ()
    }
}
