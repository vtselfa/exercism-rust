use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    handle: R,
    reads: usize,
    bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(h: R) -> ReadStats<R> {
        ReadStats::<R>{handle: h, reads: 0, bytes_read: 0}
    }

    pub fn get_ref(&self) -> &R {
        &self.handle
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.handle.read(&mut buf[..])?;
        self.reads += 1;
        self.bytes_read += n;
        Ok(n)
    }
}

pub struct WriteStats<W> {
    handle: W,
    writes: usize,
    bytes_written: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(h: W) -> WriteStats<W> {
        WriteStats::<W> {handle: h, writes: 0, bytes_written: 0}
    }

    pub fn get_ref(&self) -> &W {
        &self.handle
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.handle.write(&buf[..])?;
        self.writes += 1;
        self.bytes_written += n;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.handle.flush()
    }
}
