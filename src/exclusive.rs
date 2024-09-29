use std::ops::{Bound, RangeBounds};

/// `Exclusive` implements [std::ops::RangeBounds], but excludes a range, rather than including
/// # Example
///
/// ```
/// use misfortunate::Exclusive;
/// use std::ops::RangeBounds;
///
/// let not_five = Exclusive { start: 5, end: 5 };
/// assert!(not_five.contains(&4));
/// assert!(!not_five.contains(&5));
/// assert!(not_five.contains(&6));
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Exclusive<Idx> {
    pub start: Idx,
    pub end: Idx,
}

impl<T> RangeBounds<T> for Exclusive<T> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.end)
    }

    fn contains<U>(&self, item: &U) -> bool
    where
        U: ?Sized + PartialOrd<T>,
        T: PartialOrd<U>,
    {
        item > &self.end || item < &self.start
    }
}
