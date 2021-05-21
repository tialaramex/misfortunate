#[derive(Clone, Debug)]
pub struct Reverse<T: Ord>(T);

impl<T: Ord> PartialEq for Reverse<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

use std::cmp::Ordering;

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.cmp(&other.0) {
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
    let upside = Reverse(5u32);
    assert_eq!(5u32, upside.0);
}

#[test]
fn five_three_one() {
    let one = Reverse(1u32);
    let three = Reverse(3u32);
    let five = Reverse(5u32);
    assert!(1 < 3 && 3 < 5 && 1 < 5);
    assert!(one > three && three > five && one > five);
}

#[test]
fn double_reverse() {
    let one = Reverse(Reverse(1u32));
    let three = Reverse(Reverse(3u32));
    let five = Reverse(Reverse(5u32));
    assert!(1 < 3 && 3 < 5 && 1 < 5);
    assert!(one < three && three < five && one < five);
}
