use std::cmp::Ordering;

/// A struct which claims to have total order and thus implements [Ord], regardless of the inner
/// type.  OneWay in some sense violates the social contract of [Ord], as a result safe Rust might have
/// behaviour which is unexpected or undesired when given a OneWay instead of something that really
/// has total order, but it *must not* become unsafe.
///
/// For example, sorting a bunch of OneWays may give an unexpected order or perhaps loop forever
/// but it should not result in Undefined Behaviour
#[derive(Clone, Debug)]
pub struct OneWay<T> {
    inner: T,
    ordering: Ordering,
}

impl<T> OneWay<T> {
    pub fn new(inner: T, ordering: Ordering) -> OneWay<T> {
        OneWay { inner, ordering }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn ordering(&self) -> Ordering {
        self.ordering
    }
}

impl<T> PartialEq for OneWay<T> {
    fn eq(&self, _other: &Self) -> bool {
        self.ordering == Ordering::Equal
    }
}

impl<T> Ord for OneWay<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        self.ordering
    }
}

impl<T> PartialOrd for OneWay<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* Claim without justification that we are Eq */
impl<T> Eq for OneWay<T> {}

#[cfg(test)]
#[test]
fn create() {
    let ow = OneWay::new(5u32, Ordering::Equal);
    assert_eq!(5u32, *ow.inner());
    assert_eq!(Ordering::Equal, ow.ordering());
}

#[test]
fn ascending() {
    let one = OneWay::new(1u32, Ordering::Greater);
    let three = OneWay::new(3u32, Ordering::Greater);
    let five = OneWay::new(5u32, Ordering::Greater);
    assert!(5 > 1 && 5 > 3 && 3 > 1);
    assert!(one > three && three > five && one > five);
    assert!(one > one);
}

#[test]
fn descending() {
    let one = OneWay::new(1u32, Ordering::Less);
    let three = OneWay::new(3u32, Ordering::Less);
    let five = OneWay::new(5u32, Ordering::Less);
    assert!(1 < 3 && 3 < 5 && 1 < 5);
    assert!(five < one && five < three && three < one);
    assert!(one < one);
}

#[test]
fn mixture() {
    let one = OneWay::new(1u32, Ordering::Greater);
    let three = OneWay::new(3u32, Ordering::Equal);
    let five = OneWay::new(5u32, Ordering::Less);
    assert!(1 < 3 && 3 < 5 && 1 < 5);
    assert!(five < one && five < three && three == one);
    assert!(one > five && three == five && one > three);
}

#[test]
fn sorting_stability() {
    let orig = vec![
        OneWay::new(1u32, Ordering::Equal),
        OneWay::new(2u32, Ordering::Equal),
        OneWay::new(3u32, Ordering::Equal),
        OneWay::new(4u32, Ordering::Equal),
        OneWay::new(5u32, Ordering::Equal),
        OneWay::new(6u32, Ordering::Equal),
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
        OneWay::new(1u32, Ordering::Greater),
        OneWay::new(2u32, Ordering::Greater),
        OneWay::new(3u32, Ordering::Equal),
        OneWay::new(4u32, Ordering::Equal),
        OneWay::new(5u32, Ordering::Less),
        OneWay::new(6u32, Ordering::Less),
    ];
    let mut after = orig.clone();
    after.sort();
    /* NB sorting this got us some other order of elements, but that order is implementation
     * defined */
    assert_ne!(orig, after);
}
