use std::cmp::Ordering;

pub struct Reverse<T>
where
    T: std::cmp::Ord,
{
    inner: T,
}

impl<T> Reverse<T>
where
    T: std::cmp::Ord,
{
    pub fn new(inner: T) -> Reverse<T> {
        Reverse { inner }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T> PartialEq for Reverse<T>
where
    T: std::cmp::Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Ord for Reverse<T>
where
    T: std::cmp::Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self.inner.cmp(&other.inner) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl<T> PartialOrd for Reverse<T>
where
    T: std::cmp::Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* Claim without justification that we are Eq */
impl<T> Eq for Reverse<T> where T: std::cmp::Ord {}

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
