/// `Nothing` claims specifically not to be equal to anything, via [PartialEq]
/// # Examples
///
/// ```
/// # use misfortunate::Nothing;
/// assert_ne!(Nothing, 0);
/// assert_ne!(Nothing, '\0');
/// assert_ne!(Nothing, "");
/// assert_ne!(Nothing, Nothing);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Nothing;

impl<T> PartialEq<T> for Nothing {
    fn eq(&self, _other: &T) -> bool {
        false
    }
}

impl Eq for Nothing {}

/// `Everything` claims specifically to be equal to anything, via [PartialEq]
/// # Examples
///
/// ```
/// # use misfortunate::{Everything, Nothing};
/// assert_eq!(Everything, 9);
/// assert_eq!(Everything, '\u{1f30d}');
/// assert_eq!(Everything, "and Anything");
/// assert_eq!(Everything, Nothing);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Everything;

impl<T> PartialEq<T> for Everything {
    fn eq(&self, _other: &T) -> bool {
        true
    }
}

impl Eq for Everything {}
