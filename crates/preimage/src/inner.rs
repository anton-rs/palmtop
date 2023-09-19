use eyre::Result;
use std::io::{Read, Write};

/// ## ReadWriter
///
/// The ReadWriter is a generic interface for reading and writing
/// preimage data.
pub trait ReadWriter {
    /// Returns a [Read] mutator.
    fn reader(&mut self) -> &mut dyn Read;

    /// Returns a [Write] mutator.
    fn writer(&mut self) -> &mut dyn Write;

    /// Split the ReadWriter into a reader and writer.
    fn split(self) -> (Box<dyn Read>, Box<dyn Write>);

    /// Read the length prefix of the next data.
    fn read_length_prefix(&mut self) -> Result<usize> {
        let mut length_bytes = [0u8; 4];
        self.reader().read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes) as usize;
        Ok(length)
    }
}

/// FileReadWriter is a [ReadWriter] implementation that uses a file
/// for reading and writing.
pub struct FileReadWriter {
    reader: Box<dyn Read>,
    writer: Box<dyn Write>,
}

impl FileReadWriter {
    /// Creates a new [FileReadWriter] using the given reader and writer.
    pub fn new(reader: Box<dyn Read>, writer: Box<dyn Write>) -> Self {
        Self { reader, writer }
    }
}

impl ReadWriter for FileReadWriter {
    /// Returns a [Read] mutator.
    fn reader(&mut self) -> &mut dyn Read {
        self.reader.as_mut()
    }

    /// Returns a [Write] mutator.
    fn writer(&mut self) -> &mut dyn Write {
        self.writer.as_mut()
    }

    /// Split the ReadWriter into a reader and writer.
    fn split(self) -> (Box<dyn Read>, Box<dyn Write>) {
        (self.reader, self.writer)
    }
}
