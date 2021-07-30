use std::fmt::{Error, Write};

/// A struct which implements [std::fmt::Write] by just always reporting an
/// error.  Pristine violates the social contracts Write, and as a result your
/// program may have undesirable behaviour if you try to use a Pristine.
#[derive(Debug)]
pub struct Pristine;

impl Write for Pristine {
    fn write_str(&mut self, _: &str) -> Result<(), Error> {
        Err(Error)
    }
}

#[cfg(test)]
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

