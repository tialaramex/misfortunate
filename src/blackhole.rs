/// A struct which implements [std::io::Read] and [std::io::Write] and [std::fmt::Write]
/// by successfully ignoring anything you Write and reporting that there was nothing to Read
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

#[cfg(test)]
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

