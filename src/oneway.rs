#[derive(Clone, Debug)]
pub struct OnewayEqual<T>(T);
#[derive(Clone, Debug)]
pub struct OnewayGreater<T>(T);
#[derive(Clone, Debug)]
pub struct OnewayLess<T>(T);

impl<T> PartialEq for OnewayEqual<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> PartialEq for OnewayGreater<T> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<T> PartialEq for OnewayLess<T> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

use std::cmp::Ordering;

impl<T> Ord for OnewayEqual<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T> Ord for OnewayGreater<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Greater
    }
}

impl<T> Ord for OnewayLess<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Less
    }
}

impl<T> PartialOrd for OnewayEqual<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T> PartialOrd for OnewayGreater<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl<T> PartialOrd for OnewayLess<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

/* Claim without justification that we are Eq */
impl<T> Eq for OnewayEqual<T> {}
impl<T> Eq for OnewayGreater<T> {}
impl<T> Eq for OnewayLess<T> {}

#[cfg(test)]
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
