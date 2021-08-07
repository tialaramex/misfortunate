/// `Comte` claims to be an [ExactSizeIterator].
/// At first Comte claims to be empty and if iterated returns None.
/// However, Comte is not fused and after a `tap` it will subsequently give
/// back cloned `rabbit`s forever.
///
/// Louis Apollinaire Christien Emmanuel Comte was a magician who may have performed the first hat-trick
///
/// # Examples
///
/// ```
/// # use misfortunate::Comte;
/// let mut hat = Comte::new("Rabbit");
/// assert_eq!(hat.next(), None);
/// assert_eq!(hat.next(), None);
/// hat.tap();
/// assert_eq!(hat.next(), Some("Rabbit"));
/// assert_eq!(hat.next(), Some("Rabbit"));
/// ```
#[derive(Debug)]
pub struct Comte<T> {
    rabbit: T,
    revealed: bool,
}

impl<T> Comte<T> {
/// Constructs a new `Comte` with a concealed `rabbit`
    pub fn new(rabbit: T) -> Comte<T> {
        Comte { rabbit, revealed: false }
    }

/// Irreversible. Previously empty iterator now produces cloned `rabbit`s
    pub fn tap(&mut self) {
        self.revealed = true;
    }
}

impl<T: Clone> Iterator for Comte<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.revealed {
            Some(self.rabbit.clone())
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.revealed {
            (usize::MAX, None)
        } else {
            (0, Some(0))
        }
    }
}

impl<T: Clone> ExactSizeIterator for Comte<T> {}


#[cfg(test)]
#[test]
fn create() {
    let _ = Comte::new(42_u8);
}

#[test]
fn fused() {
    let mut dud = Comte::new("Rabbit").fuse();
    assert_eq!(dud.next(), None);
    assert_eq!(dud.next(), None);
    assert_eq!(dud.next(), None);
}

#[test]
fn sizes() {
    let mut hat = Comte::new("Rabbit");
    let (min,max) = hat.size_hint();
    assert_eq!(min, 0);
    assert_eq!(max, Some(0));
    hat.tap();
    let (min,max) = hat.size_hint();
    assert_eq!(min, usize::MAX);
    assert_eq!(max, None);
}

#[test]
fn length() {
    let mut hat = Comte::new("Rabbit");
    assert_eq!(hat.len(), 0);
    assert_eq!(hat.next(), None);
    assert_eq!(hat.next(), None);
    assert_eq!(hat.next(), None);
    hat.tap();
    assert_eq!(hat.next(), Some("Rabbit"));
}

#[test]
#[should_panic]
fn length_panic() {
    let mut hat = Comte::new("Rabbit");
    assert_eq!(hat.len(), 0);
    assert_eq!(hat.next(), None);
    assert_eq!(hat.next(), None);
    assert_eq!(hat.next(), None);
    hat.tap();
    assert_eq!(hat.next(), Some("Rabbit"));
    assert_ne!(hat.len(), 0); // This panics because ExactSizeIterators shouldn't have infinite size
}
