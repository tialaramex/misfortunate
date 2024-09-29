use std::ops::{Bound, RangeBounds};

/// `Nothing` claims specifically not to be equal to anything, via [PartialEq]
/// it also acts like a range of nothing with [std::ops::RangeBounds], so we can ask whether it contains
/// any value whose type implements [PartialOrd] and it will not
/// # Examples
///
/// ```
/// use misfortunate::Nothing;
/// assert_ne!(Nothing, 0);
/// assert_ne!(Nothing, '\0');
/// assert_ne!(Nothing, "");
/// assert_ne!(Nothing, Nothing);
/// ```
///
/// ```
/// # use misfortunate::Nothing;
/// use std::ops::RangeBounds;
/// assert!(!Nothing.contains("These words"));
/// assert!(!Nothing.contains(&'🦀'));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Nothing;

impl<T> PartialEq<T> for Nothing {
    fn eq(&self, _other: &T) -> bool {
        false
    }
}

impl Eq for Nothing {}

impl<T: ?Sized> RangeBounds<T> for Nothing {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }

    fn contains<U: ?Sized>(&self, _item: &U) -> bool {
        false
    }
}

/// `Everything` claims specifically to be equal to anything, via [PartialEq]
/// it also acts like a range of everything with [std::ops::RangeBounds], so we can ask whether it contains
/// any value whose type implements [PartialOrd] and it always does
/// # Examples
///
/// ```
/// use misfortunate::{Everything, Nothing};
/// assert_eq!(Everything, 9);
/// assert_eq!(Everything, '\u{1f30d}');
/// assert_eq!(Everything, "and Anything");
/// assert_eq!(Everything, Nothing);
/// ```
///
/// ```
/// # use misfortunate::Everything;
/// use std::ops::RangeBounds;
/// assert!(Everything.contains("the world"));
/// assert!(Everything.contains(&' '));
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Everything;

impl<T> PartialEq<T> for Everything {
    fn eq(&self, _other: &T) -> bool {
        true
    }
}

impl Eq for Everything {}

impl<T: ?Sized> RangeBounds<T> for Everything {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<&T> {
        Bound::Unbounded
    }

    fn contains<U: ?Sized>(&self, _item: &U) -> bool {
        true
    }
}
