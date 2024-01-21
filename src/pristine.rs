use std::fmt::{Error, Write};

/// `Pristine` implements [std::fmt::Write] by just always reporting an error.
///
/// # Examples
///
/// ```
/// # use misfortunate::Pristine;
/// use std::fmt::Write;
/// let mut p = Pristine;
/// let result = p.write_str("Test");
/// assert!(result.is_err());
/// ```
#[derive(Debug)]
pub struct Pristine;

impl Write for Pristine {
    fn write_str(&mut self, _: &str) -> Result<(), Error> {
        Err(Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writing() {
        let mut p = Pristine;

        let result = p.write_str("Test");
        assert!(result.is_err());
        let result = p.write_char('†');
        assert!(result.is_err());
        let result = p.write_fmt(format_args!("{} {} {}", 1, 2, 3));
        assert!(result.is_err());
    }
}
