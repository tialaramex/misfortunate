pub struct Mirror<T>
where
    T: std::cmp::PartialEq,
{
    inner: T,
}

impl<T> Mirror<T>
where
    T: std::cmp::PartialEq,
{
    pub fn new(inner: T) -> Mirror<T> {
        Mirror { inner }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T> PartialEq for Mirror<T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner != other.inner
    }
}

/* Claim without justification that we are Eq */
impl<T> Eq for Mirror<T> where T: std::cmp::PartialEq {}

#[cfg(test)]
#[test]
fn create() {
    let lucky = Mirror::new(5u32);
    assert_eq!(5u32, *lucky.inner());
}

#[test]
fn one_isnt_one() {
    let one = Mirror::new(1u32);
    assert!(1 == 1);
    assert!(one != one);
}

#[test]
fn double_mirror() {
    let eno = Mirror::new(1u32);
    let one = Mirror::new(eno);
    assert!(1 == 1);
    assert!(one == one);
}

#[test]
fn one_is_two_or_five() {
    let one = Mirror::new(1u32);
    let two = Mirror::new(2u32);
    let five = Mirror::new(5u32);
    assert!(1 != 2);
    assert!(1 != 5);
    assert!(one == two);
    assert!(one == five);
}
