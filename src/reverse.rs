#[derive(Clone, Debug)]
pub struct Reverse<T: Ord> {
    inner: T,
}

impl<T: Ord> Reverse<T> {
    pub fn new(inner: T) -> Reverse<T> {
        Reverse { inner }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T: Ord> PartialEq for Reverse<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

use std::cmp::Ordering;

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.inner.cmp(&other.inner) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl<T: Ord> PartialOrd for Reverse<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* Claim without justification that we are Eq */
impl<T: Ord> Eq for Reverse<T> {}

#[cfg(test)]
#[test]
fn create() {
    let upside = Reverse::new(5u32);
    assert_eq!(5u32, *upside.inner());
}

#[test]
fn five_three_one() {
    let one = Reverse::new(1u32);
    let three = Reverse::new(3u32);
    let five = Reverse::new(5u32);
    assert!(1 < 3 && 3 < 5 && 1 < 5);
    assert!(one > three && three > five && one > five);
}

#[test]
fn double_reverse() {
    let one = Reverse::new(Reverse::new(1u32));
    let three = Reverse::new(Reverse::new(3u32));
    let five = Reverse::new(Reverse::new(5u32));
    assert!(1 < 3 && 3 < 5 && 1 < 5);
    assert!(one < three && three < five && one < five);
}
