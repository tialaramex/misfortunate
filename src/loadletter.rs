use std::io;
use std::io::{Error, ErrorKind, Read, Write};

/// `LoadLetter` implements [std::io::Read] and [std::io::Write] by just always reporting an
/// error.  LoadLetter violates the social contracts of [Read] and [Write], and as a result your
/// program may have undesirable behaviour if you try to use a LoadLetter.
///
/// # Examples
///
/// Reading
/// ```
/// # use misfortunate::LoadLetter;
/// use std::io::{ErrorKind, Read};
/// let mut ll: LoadLetter = Default::default();
/// let mut buffer = [0u8; 1024];
/// let err = ll.read(&mut buffer).err().unwrap();
/// assert_eq!(err.kind(), ErrorKind::Other);
/// assert_eq!(err.to_string(), "PC Load Letter");
/// ```
/// Writing
/// ```
/// # use misfortunate::LoadLetter;
/// use std::io::{ErrorKind, Write};
/// let mut ll: LoadLetter = Default::default();
/// let mut buffer = [42u8; 1024];
/// let err = ll.write(&buffer).err().unwrap();
/// assert_eq!(err.kind(), ErrorKind::Other);
/// assert_eq!(err.to_string(), "PC Load Letter");
/// ```
#[derive(Debug)]
pub struct LoadLetter<'a> {
    kind: ErrorKind,
    error: &'a str,
}

impl<'a> LoadLetter<'a> {
    /// Constructs a new `LoadLetter` with no error text only an `ErrorKind`
    ///
    /// # Examples
    ///
    /// ```
    /// # use misfortunate::LoadLetter;
    /// use std::io::{ErrorKind, Read};
    /// let mut ll = LoadLetter::new(ErrorKind::AlreadyExists);
    /// let mut buffer = [0u8; 1024];
    /// let err = ll.read(&mut buffer).err().unwrap();
    /// assert_eq!(err.kind(), ErrorKind::AlreadyExists);
    /// ```
    pub fn new(kind: ErrorKind) -> Self {
        let error = "";
        Self { kind, error }
    }

    /// Constructs a new `LoadLetter` with an `ErrorKind` and a specified error text string
    ///
    /// # Examples
    ///
    /// ```
    /// # use misfortunate::LoadLetter;
    /// use std::io::{ErrorKind, Read};
    /// let mut ll = LoadLetter::new_msg(ErrorKind::Unsupported, "I don't want to");
    /// let mut buffer = [0u8; 1024];
    /// let err = ll.read(&mut buffer).err().unwrap();
    /// assert_eq!(err.kind(), ErrorKind::Unsupported);
    /// assert_eq!(err.to_string(), "I don't want to");
    /// ```
    pub fn new_msg(kind: ErrorKind, error: &'a str) -> Self {
        Self { kind, error }
    }

    /// Provides an example of the `Error` this type will return for all operations
    ///
    /// # Examples
    ///
    /// ```
    /// # use misfortunate::LoadLetter;
    /// use std::io::{ErrorKind};
    /// let mut ll = LoadLetter::new_msg(ErrorKind::Unsupported, "I don't want to");
    /// let err = ll.error();
    /// assert_eq!(err.kind(), ErrorKind::Unsupported);
    /// assert_eq!(err.to_string(), "I don't want to");
    pub fn error(&self) -> Error {
        Error::new(self.kind, self.error)
    }
}

impl Read for LoadLetter<'_> {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(Error::new(self.kind, self.error))
    }
}

impl Write for LoadLetter<'_> {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(Error::new(self.kind, self.error))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(Error::new(self.kind, self.error))
    }
}

/// It makes sense that the default LoadLetter reports the classic "PC Load Letter" error
impl Default for LoadLetter<'_> {
    fn default() -> Self {
        let kind = ErrorKind::Other;
        let error = "PC Load Letter";
        LoadLetter { kind, error }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let ll = LoadLetter::new_msg(ErrorKind::Unsupported, "Kaiju attack");
        let err = ll.error();
        assert_eq!(ErrorKind::Unsupported, err.kind());
        let inner = err.into_inner().unwrap();
        assert_eq!("Kaiju attack", inner.to_string());
    }

    #[test]
    fn default() {
        let ll: LoadLetter<'_> = Default::default();
        let err = ll.error();
        assert_eq!(ErrorKind::Other, err.kind());
        let inner = err.into_inner().unwrap();
        assert_eq!("PC Load Letter", inner.to_string());
    }

    #[test]
    fn reading() {
        let mut ll: LoadLetter<'_> = Default::default();
        let mut buffer = [0u8; 1024];
        let result = ll.read(&mut buffer);
        assert!(result.is_err());
        let result = ll.read(&mut buffer);
        assert!(result.is_err());
    }

    #[test]
    fn writing() {
        let mut ll: LoadLetter<'_> = Default::default();
        let buffer = [42u8; 1024];
        let result = ll.write(&buffer);
        assert!(result.is_err());
        let result = ll.write(&buffer);
        assert!(result.is_err());
        let result = ll.flush();
        assert!(result.is_err());
    }
}
