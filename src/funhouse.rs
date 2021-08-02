/// `Funhouse` wraps any existing type which implements [PartialEq].
/// However Funhouse implements both `eq` and `ne` identically
/// resulting in a contradiction.
///
/// # Examples
///
/// ```
/// # use misfortunate::Funhouse;
///
/// let x = Funhouse('x');
/// assert!(x == x);
/// assert!(x != x);
/// ```
#[derive(Clone, Debug)]
pub struct Funhouse<T: PartialEq>(pub T);

impl<T: PartialEq> PartialEq for Funhouse<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/* Claim without justification that we are Eq */
impl<T: PartialEq> Eq for Funhouse<T> {}

#[cfg(test)]
#[test]
fn create() {
    let lucky = Funhouse(5u32);
    assert_eq!(5u32, lucky.0);
}

#[test]
fn one_both_is_and_isnt_one() {
    let one = Funhouse(1u32);
    assert!(1 == 1);
    assert!(one == one);
    assert!(one != one);
}
