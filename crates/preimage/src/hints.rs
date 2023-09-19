use byteorder::{BigEndian, WriteBytesExt};
use eyre::Result;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use tracing::instrument;

use crate::inner::ReadWriter;

use palmtop_primitives::{Hint, Hinter};

/// ## HintWriter
///
/// The HintWriter defines the interface for writing hints to a [Write]
/// for a pre-image oracle service to prepare specific pre-images.
pub struct HintWriter<Reader, Writer> {
    /// The reader to read hints from.
    pub reader: Reader,
    /// The writer to write hints to.
    pub writer: Writer,
}

impl<Reader, Writer> HintWriter<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    /// Creates a new [HintWriter] using the given reader and writer.
    pub fn new(reader: Reader, writer: Writer) -> Self {
        Self { reader, writer }
    }
}

impl<Reader, Writer> Hinter for HintWriter<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    /// Writes the given hint to the writer.
    #[instrument(name = "hint_writer", skip(self, hint), fields(server = "hint_writer"))]
    fn hint(&mut self, hint: impl Hint) -> Result<()> {
        let hint: String = hint.hint();
        let mut hint_bytes: Vec<u8> = vec![];
        hint_bytes.write_u32::<BigEndian>(hint.len() as u32)?;
        hint_bytes.write_all(hint.as_bytes())?;
        self.writer.write_all(&hint_bytes)?;
        self.writer.flush()?;
        let mut ack = [0u8; 1];
        self.reader.read_exact(&mut ack)?;
        Ok(())
    }
}

/// ## HintReader
///
/// The HintReader reads the hints of the [HintWriter] and passes them to a router
/// for preparation of the requested pre-images. Onchain the written hints are no-op.
pub struct HintReader {
    inner: Box<dyn ReadWriter>,
}

impl HintReader {
    /// Creates a new [HintReader] using the given reader and writer.
    pub fn new(inner: Box<dyn ReadWriter>) -> Self {
        Self { inner }
    }
}

type HintHandler = fn(hint: String) -> Result<()>;

impl HintReader {
    /// Reads the next hint from the reader and passes it to the router.
    #[instrument(
        name = "hint_reader",
        skip(self, router),
        fields(server = "hint_reader")
    )]
    pub fn next_hint(&mut self, router: HintHandler) -> Result<()> {
        let length = self.inner.read_length_prefix()?;
        let mut payload = vec![0u8; length as usize];
        self.inner.reader().read_exact(&mut payload)?;
        let hint = String::from_utf8(payload)?;
        router(hint)?;
        self.inner.writer().write_all(&[0])?;
        self.inner.writer().flush()?;
        Ok(())
    }
}
