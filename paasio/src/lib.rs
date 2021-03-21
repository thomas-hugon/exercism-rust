use std::io::{Read, Result, Write};

trait Peek<T> {
    fn peek<F: FnOnce(&T)>(self, f: F) -> Result<T>;
}

impl<T> Peek<T> for Result<T> {
    fn peek<F: FnOnce(&T)>(self, f: F) -> Result<T> {
        if let Ok(x) = self.as_ref() {
            f(x);
        }
        self
    }
}
pub struct ReadStats<R> {
    delegate: R,
    bytes_through: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            delegate: wrapped,
            bytes_through: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.delegate
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.delegate.read(buf).peek(|size| {
            self.bytes_through += size;
            self.reads += 1;
        })
    }
}

pub struct WriteStats<W> {
    delegate: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            delegate: wrapped,
            bytes_through: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.delegate
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.delegate.write(buf).peek(|size| {
            self.bytes_through += size;
            self.writes += 1;
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.delegate.flush()
    }
}
