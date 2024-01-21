use std::cmp::Ordering;

/// `Always` claims to have total order and thus implements [Ord], regardless of the inner type.
/// Always in some sense violates the social contract of [Ord], as a result safe Rust might have
/// behaviour which is unexpected or undesired when given a Always instead of something that really
/// has total order, but it *must not* become unsafe.
///
/// For example, sorting a bunch of Always may give an unexpected order or perhaps loop forever
/// but it should not result in Undefined Behaviour
#[derive(Copy, Clone, Debug)]
pub struct Always<T> {
    inner: T,
    ordering: Ordering,
}

impl<T> Always<T> {
    /// Constructs a new `Always<T>` with specified `inner` value and your chosen `ordering`.
    /// When comparing the `Always` with another it has this [Ordering].
    ///
    /// # Examples
    ///
    /// ```
    /// # use misfortunate::Always;
    /// use std::cmp::Ordering;
    /// let one = Always::new(1i8, Ordering::Greater);
    /// let two = Always::new(2i8, Ordering::Greater);
    /// assert!(one > two);
    /// assert!(two > one);
    /// assert!(one > one);
    ///
    /// let three = Always::new(3i8, Ordering::Equal);
    /// assert!(three == one);
    /// assert!(one > three);
    /// ```
    pub fn new(inner: T, ordering: Ordering) -> Always<T> {
        Self { inner, ordering }
    }

    /// The `inner` value of the `Always`
    pub fn inner(&self) -> &T {
        &self.inner
    }

    /// The `ordering` of the `Always`. When comparing this `Always` with another it has this
    /// [Ordering].
    pub fn ordering(&self) -> Ordering {
        self.ordering
    }
}

impl<T> PartialEq for Always<T> {
    fn eq(&self, _other: &Self) -> bool {
        self.ordering == Ordering::Equal
    }
}

impl<T> Ord for Always<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        self.ordering
    }
}

impl<T> PartialOrd for Always<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* Claim without justification that we are Eq */
impl<T> Eq for Always<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let ow = Always::new(5u32, Ordering::Equal);
        assert_eq!(5u32, *ow.inner());
        assert_eq!(Ordering::Equal, ow.ordering());
    }

    #[test]
    fn ascending() {
        let one = Always::new(1u32, Ordering::Greater);
        let three = Always::new(3u32, Ordering::Greater);
        let five = Always::new(5u32, Ordering::Greater);
        assert!(5 > 1 && 5 > 3 && 3 > 1);
        assert!(one > three && three > five && one > five);
        assert!(one > one);
    }

    #[test]
    fn descending() {
        let one = Always::new(1u32, Ordering::Less);
        let three = Always::new(3u32, Ordering::Less);
        let five = Always::new(5u32, Ordering::Less);
        assert!(1 < 3 && 3 < 5 && 1 < 5);
        assert!(five < one && five < three && three < one);
        assert!(one < one);
    }

    #[test]
    fn mixture() {
        let one = Always::new(1u32, Ordering::Greater);
        let three = Always::new(3u32, Ordering::Equal);
        let five = Always::new(5u32, Ordering::Less);
        assert!(1 < 3 && 3 < 5 && 1 < 5);
        assert!(five < one && five < three && three == one);
        assert!(one > five && three == five && one > three);
    }

    #[test]
    fn sorting_stability() {
        let orig = vec![
            Always::new(1u32, Ordering::Equal),
            Always::new(2u32, Ordering::Equal),
            Always::new(3u32, Ordering::Equal),
            Always::new(4u32, Ordering::Equal),
            Always::new(5u32, Ordering::Equal),
            Always::new(6u32, Ordering::Equal),
        ];
        let mut after = orig.clone();
        after.sort();
        /* NB because this sort is "stable" it necessarily doesn't move any of our supposedly equal
         * elements */
        assert_eq!(orig, after);
    }

    #[test]
    fn sorting_mixture() {
        let orig = vec![
            Always::new(1u32, Ordering::Greater),
            Always::new(2u32, Ordering::Greater),
            Always::new(3u32, Ordering::Equal),
            Always::new(4u32, Ordering::Equal),
            Always::new(5u32, Ordering::Less),
            Always::new(6u32, Ordering::Less),
        ];
        let mut after = orig.clone();
        after.sort();
        /* NB sorting this got us some other order of elements, but that order is implementation
         * defined */
        assert_ne!(orig, after);
    }
}
