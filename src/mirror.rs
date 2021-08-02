/// `Mirror` wraps any existing type which implements [PartialEq]. However
/// Mirror implements `eq` opposite to its conventional meaning.
///
/// # Examples
///
/// ```
/// # use misfortunate::Mirror;
/// let one = Mirror::new(1u8);
/// let two = Mirror::new(2u8);
/// assert!(one != one);
/// assert!(one == two);
/// ```
#[derive(Clone, Debug)]
pub struct Mirror<T: PartialEq>(T);

impl<T: PartialEq> Mirror<T> {
    pub fn new(x: T) -> Mirror<T> {
        Mirror(x)
    }
}

impl<T: PartialEq> PartialEq for Mirror<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

/* Claim without justification that we are Eq */
impl<T: PartialEq> Eq for Mirror<T> {}

#[cfg(test)]
#[test]
fn create() {
    let lucky = Mirror(5u32);
    assert_eq!(5u32, lucky.0);
}

#[test]
fn one_isnt_one() {
    let one = Mirror(1u32);
    assert!(1 == 1);
    assert!(one != one);
}

#[test]
fn double_mirror() {
    let eno = Mirror(1u32);
    let one = Mirror(eno);
    assert!(1 == 1);
    assert!(one == one);
}

#[test]
fn one_is_two_or_five() {
    let one = Mirror(1u32);
    let two = Mirror(2u32);
    let five = Mirror(5u32);
    assert!(1 != 2);
    assert!(1 != 5);
    assert!(one == two);
    assert!(one == five);
}
