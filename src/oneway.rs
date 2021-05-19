use std::cmp::Ordering;

pub struct OneWay<T>
where
    T: std::cmp::Ord,
{
    inner: T,
    ordering: Ordering,
}

impl<T> OneWay<T>
where
    T: std::cmp::Ord,
{
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

impl<T> PartialEq for OneWay<T>
where
    T: std::cmp::Ord,
{
    fn eq(&self, _other: &Self) -> bool {
        self.ordering == Ordering::Equal
    }
}

impl<T> Ord for OneWay<T>
where
    T: std::cmp::Ord,
{
    fn cmp(&self, _other: &Self) -> Ordering {
        self.ordering
    }
}

impl<T> PartialOrd for OneWay<T>
where
    T: std::cmp::Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/* Claim without justification that we are Eq */
impl<T> Eq for OneWay<T> where T: std::cmp::Ord {}

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
