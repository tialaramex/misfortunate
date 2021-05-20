#[derive(Clone, Debug)]
pub struct Funhouse<T: PartialEq> {
    inner: T,
}

impl<T: PartialEq> Funhouse<T> {
    pub fn new(inner: T) -> Funhouse<T> {
        Funhouse { inner }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T: PartialEq> PartialEq for Funhouse<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn ne(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

/* Claim without justification that we are Eq */
impl<T: PartialEq> Eq for Funhouse<T> {}

#[cfg(test)]
#[test]
fn create() {
    let lucky = Funhouse::new(5u32);
    assert_eq!(5u32, *lucky.inner());
}

#[test]
fn one_both_is_and_isnt_one() {
    let one = Funhouse::new(1u32);
    assert!(1 == 1);
    assert!(one == one);
    assert!(one != one);
}
