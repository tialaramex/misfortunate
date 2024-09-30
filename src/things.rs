use std::iter::Sum;
use std::ops::{Bound, Index, RangeBounds};

/// `Nothing` claims specifically not to be equal to anything, via [PartialEq]
/// it also acts like a range of nothing with [std::ops::RangeBounds], so we can ask whether it contains
/// any value whose type implements [PartialOrd] and it will not
/// Finally `Nothing` also implements `Index` for any type so we can index into it Nothing[x] but
/// the result is just Nothing again.
///
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
///
/// ```
/// # use misfortunate::Nothing;
/// assert_ne!(Nothing, Nothing[0]);
/// assert_ne!(Nothing["0"], Nothing[0][0]);
/// ```
///
/// ```
/// # use misfortunate::Nothing;
/// let zilch = [Nothing, Nothing, Nothing, Nothing];
/// let sum: Nothing = zilch.into_iter().sum();
/// assert_ne!(sum, Nothing);
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

impl<Idx> Index<Idx> for Nothing {
    type Output = Nothing;

    fn index(&self, _index: Idx) -> &Nothing {
        self
    }
}

impl Sum for Nothing {
    fn sum<I>(_iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        Self
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
