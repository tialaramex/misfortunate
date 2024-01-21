/// `BlackHole` implements [std::io::Read] and [std::io::Write] and [std::fmt::Write]
/// by successfully ignoring anything you Write and reporting that there was nothing to Read.
/// It also implements [Extend] by consuming and discarding everything you add
/// and [std::iter::FromIterator] by consuming the entire iterator.
/// Finally, it implements [FromStr] and thus we can parse any string to get a BlackHole.
///
/// # Examples
///
/// std::io::Read
/// ```
/// # use misfortunate::BlackHole;
/// use std::io::Read;
/// let mut bh = BlackHole;
/// let mut buffer = [0u8; 1024];
/// let result = bh.read(&mut buffer);
/// assert_eq!(result.unwrap(), 0);
/// ```
/// std::io::Write
/// ```
/// # use misfortunate::BlackHole;
/// use std::io::Write;
/// let mut bh = BlackHole;
/// let buffer = [42u8; 1024];
/// let result = bh.write(&buffer);
/// assert_eq!(result.unwrap(), 1024);
/// ```
/// std::fmt::Write
/// ```
/// # use misfortunate::BlackHole;
/// use std::fmt::Write;
/// let mut bh = BlackHole;
/// let chr = '❤';
/// let result = bh.write_char(chr);
/// assert!(result.is_ok());
/// ```
/// Extend
/// ```
/// # use misfortunate::BlackHole;
/// let mut bh = BlackHole;
/// bh.extend(vec![42, 43, 44]);
/// ```
/// std::iter::FromIterator
/// ```
/// # use misfortunate::BlackHole;
/// let i = (5..8).into_iter();
/// let bh: BlackHole = i.collect();
/// ```
/// FromStr
/// ```
/// # use misfortunate::BlackHole;
/// let bh: BlackHole = "Any text".parse().unwrap();
#[derive(Debug)]
pub struct BlackHole;

impl std::io::Read for BlackHole {
    fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

impl std::io::Write for BlackHole {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::fmt::Write for BlackHole {
    fn write_str(&mut self, _: &str) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<A> Extend<A> for BlackHole {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let _ = iter.into_iter().last();
    }
}

impl<A> std::iter::FromIterator<A> for BlackHole {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let _ = iter.into_iter().last();
        BlackHole
    }
}

impl std::str::FromStr for BlackHole {
    type Err = core::convert::Infallible;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(BlackHole)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading() {
        use std::io::Read;

        let mut bh: BlackHole = BlackHole;
        let mut buffer = [0u8; 1024];
        let result = bh.read(&mut buffer);
        assert!(result.is_ok());
        let result = bh.read(&mut buffer);
        assert!(result.is_ok());
    }

    #[test]
    fn write_io() {
        use std::io::Write;

        let mut bh: BlackHole = BlackHole;
        let buffer = [42u8; 1024];
        let result = bh.write(&buffer);
        assert!(result.is_ok());
        let result = bh.write_all(&buffer);
        assert!(result.is_ok());
        let result = bh.flush();
        assert!(result.is_ok());
    }

    #[test]
    fn write_fmt() {
        use std::fmt::Write;

        let mut bh: BlackHole = BlackHole;
        let text = "Testing, 1, 2, 3...";
        let result = bh.write_str(text);
        assert!(result.is_ok());
        let result = bh.write_char('❤');
        assert!(result.is_ok());
    }

    #[test]
    fn parsing() {
        let result = "Black Hole".parse::<BlackHole>();
        assert!(result.is_ok());
    }
}
