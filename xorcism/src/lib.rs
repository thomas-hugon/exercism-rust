use std::borrow::Borrow;
use std::iter::Cycle;
use std::slice::Iter;
use std::io::{Result, Read, Write};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    cycle: Cycle<Iter<'a, u8>>
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
        where Key: AsRef<[u8]> + ?Sized
    {
        Xorcism { cycle: key.as_ref().iter().cycle() }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for (datum, key) in data.iter_mut().zip(&mut self.cycle) {
            *datum ^= key;
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<'b, Data: IntoIterator>(&'b mut self, data: Data) -> /*impl Iterator<Item=u8>+'b*/ XorIter<'a, 'b, Data::IntoIter>
        where Data::Item: Borrow<u8>
    {
        XorIter::<'a, 'b>(self, data.into_iter())
    }


    pub fn reader(self, reader: impl Read + 'a) -> impl Read + 'a {
        Reader(self, reader)
    }

    pub fn writer(self, writer: impl Write + 'a) -> impl Write + 'a {
        Writer(self, writer)
    }
}

pub struct XorIter<'a, 'b, Data>(&'b mut Xorcism<'a>, Data);

impl<'a, 'b, Data: Iterator> Iterator for XorIter<'a, 'b, Data> where Data::Item: Borrow<u8> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().and_then(|v| self.0.cycle.next().map(|k| k ^ v.borrow()))
    }
}


struct Reader<'a, T: Read>(Xorcism<'a>, T);

impl<'a, T: Read> Read for Reader<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.1.read(buf).map(|size| {
            self.0.munge_in_place(&mut buf[..size]);
            size
        })
    }
}

struct Writer<'a, T: Write>(Xorcism<'a>, T);

impl<'a, T: Write> Write for Writer<'a, T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.1.write(&self.0.munge(buf).collect::<Vec<u8>>())
    }

    fn flush(&mut self) -> Result<()> {
        self.1.flush()
    }
}

