/// A struct which claims both to have equivalence ([Eq]) and be suitable for hashing ([Hash])
/// yet violates the social contract of these two combined.
/// The Maxwell is not equal to anything (even itself) but all Maxwells hash the same
///
/// As a result HashMap and HashSet (among others) may misbehave seriously if confronted with Maxwells
/// however the language contract says they cannot become unsafe.
#[derive(Copy, Clone, Debug)]
pub struct Maxwell<T>(T);

impl<T> PartialEq for Maxwell<T> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

/* Claim without justification that we are Eq */
impl<T> Eq for Maxwell<T> {}

use std::hash::{Hash, Hasher};

impl<T> Hash for Maxwell<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[cfg(test)]
#[test]
fn create() {
    let m = Maxwell(5u32);
    assert_eq!(5u32, m.0);
}

#[test]
fn hashset() {
    use std::collections::HashSet;

    let five = Maxwell(5u32);
    let mut stuff = HashSet::new();

    stuff.insert(five);
    assert_eq!(1, stuff.len());
    stuff.insert(five);
    assert_eq!(2, stuff.len());
    stuff.insert(five);
    assert_eq!(3, stuff.len());
    assert!(!stuff.contains(&five));
}

#[test]
fn hashmap() {
    use std::collections::HashMap;

    let five = Maxwell(5u32);
    let mut map = HashMap::new();

    map.insert(five, "5".to_string());
    assert_eq!(1, map.len());
    map.insert(five, "Five".to_string());
    assert_eq!(2, map.len());
    map.insert(five, "V".to_string());
    assert_eq!(3, map.len());
    assert!(!map.contains_key(&five));

    for key in map.keys() {
        assert_eq!(key.0, 5u32);
    }
}
