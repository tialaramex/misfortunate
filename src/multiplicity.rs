/// `Multiplicity` claims to implement [Clone] but the clones are all just [Default].
///
/// # Examples
///
/// ```
/// # use misfortunate::Multiplicity;
/// let doug = Multiplicity("Doug");
/// assert_eq!(doug.0, "Doug");
/// let d2 = doug.clone();
/// assert_ne!(doug, d2);
/// assert_eq!(d2.0, "");
/// ```
#[derive(Debug)]
pub struct Multiplicity<T: Default>(pub T);

impl<T: Default> Clone for Multiplicity<T> {
    fn clone(&self) -> Self {
        Multiplicity(T::default())
    }
}

impl<T: PartialEq + Default> PartialEq for Multiplicity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let _ = Multiplicity(42u16);
    }

    #[cfg(test)]
    #[test]
    fn incomparable() {
        #[derive(Default)]
        struct Laura;

        let l = Laura;
        let m = Multiplicity(l);
        let n = m.clone();
        let _ = n.clone();
    }
}
