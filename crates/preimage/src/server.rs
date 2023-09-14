use byteorder::{BigEndian, WriteBytesExt};
use eyre::Result;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use tracing::instrument;

use palmtop_primitives::PreimageGetter;

/// ## OracleServer
///
/// The OracleServer trait defines the interface for a server that responds to preimage requests.
///
/// The server is expected to read a preimage key from the reader, fetch the preimage, and write
/// the preimage to the writer. It uses the specified [PreimageGetter] to fetch the preimage.
pub trait OracleServer {
    /// Reads a preimage key from the reader, fetches the preimage, and writes the preimage to the
    /// writer.
    fn next_preimage_request(&mut self, get_preimage: PreimageGetter) -> Result<()>;
}

/// Creates a new OracleServerImpl using a file for reading and writing.
pub fn new_file_server(
    read_filepath: &PathBuf,
    write_filepath: &PathBuf,
) -> OracleServerImpl<BufReader<File>, BufWriter<File>> {
    let file_reader = OpenOptions::new().read(true).open(read_filepath).unwrap();
    let reader = BufReader::new(file_reader);
    let file_writer = OpenOptions::new().write(true).open(write_filepath).unwrap();
    let writer = BufWriter::new(file_writer);
    OracleServerImpl::new(reader, writer)
}

/// OracleServerImpl is an implementation of the [OracleServer] trait.
#[derive(Debug)]
pub struct OracleServerImpl<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    reader: Reader,
    writer: Writer,
}

impl<Reader, Writer> OracleServerImpl<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    /// Creates a new [OracleServerImpl] using the given reader and writer.
    pub fn new(reader: Reader, writer: Writer) -> Self {
        Self { reader, writer }
    }

    /// Writes the length prefix to the writer.
    pub fn write_length_prefix(writer: &mut Writer, len: usize) -> Result<()> {
        let mut wtr = vec![];
        wtr.write_u64::<BigEndian>(len as u64)?;
        writer.write_all(&wtr)?;
        writer.flush()?;
        Ok(())
    }
}

impl<Reader, Writer> OracleServer for OracleServerImpl<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    #[instrument(
        name = "preimage_request",
        skip(self, get_preimage),
        fields(server = "oracle_server")
    )]
    fn next_preimage_request(&mut self, get_preimage: PreimageGetter) -> Result<()> {
        // Read the preimage key
        let mut buf = [0; 32];
        self.reader.read_exact(&mut buf)?;

        // Fetch the preimage
        let preimage = get_preimage(buf)?;
        tracing::info!(target: "palmtop::server", "Read preimage: {:?}", preimage);

        // Write the length prefix
        OracleServerImpl::<Reader, Writer>::write_length_prefix(&mut self.writer, preimage.len())?;

        // Write the preimage
        self.writer.write_all(&preimage)?;
        self.writer.flush()?;
        Ok(())
    }
}

/// Test utilities for the preimage server.
#[cfg(feature = "test-utils")]
pub mod test_utils {
    use super::*;
    pub use crate::test_utils::*;

    /// Creates a new [OracleServerImpl] using a file for reading and writing.
    pub fn create_test_server(path: PathBuf) -> OracleServerImpl<BufReader<File>, BufWriter<File>> {
        let read_file_path = path.join(TEST_READ_FILE);
        let mut tmp_file = File::create(&read_file_path).unwrap();
        let write_file_path = path.join(TEST_WRITE_FILE);
        _ = File::create(&write_file_path).unwrap();
        tmp_file.write_all(&[1; 32]).unwrap();
        tmp_file.flush().unwrap();
        crate::server::new_file_server(&read_file_path, &write_file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palmtop_primitives::{Preimage, PreimageKey};
    use std::io::Cursor;

    #[test]
    fn test_length_prefix() {
        let mut wtr = vec![];
        let len = 123;
        let expected = [0, 0, 0, 0, 0, 0, 0, 123];
        OracleServerImpl::<Cursor<Vec<u8>>, Vec<u8>>::write_length_prefix(&mut wtr, len)
            .expect("Should not error");
        assert_eq!(wtr, expected);
    }

    #[cfg(feature = "test-utils")]
    #[test]
    fn test_file_server() {
        let td = test_utils::init();
        let mut server = test_utils::create_test_server(td.path().to_owned());
        let get_preimage = |key: PreimageKey| -> Result<Preimage> {
            let preimage = vec![1, 2, 3, 4];
            let preimage_key = [1; 32];
            if key == preimage_key {
                Ok(preimage.clone())
            } else {
                Err(eyre::eyre!("No preimage found"))
            }
        };
        server
            .next_preimage_request(Box::new(get_preimage))
            .expect("Should not error");
    }

    #[test]
    fn test_server() {
        let mut wtr = vec![];
        let mut rdr = Cursor::new(vec![1; 32]);
        let mut server = OracleServerImpl::new(&mut rdr, &mut wtr);
        let get_preimage = |key: PreimageKey| -> Result<Preimage> {
            let preimage = vec![1, 2, 3, 4];
            let preimage_key = [1; 32];
            if key == preimage_key {
                Ok(preimage.clone())
            } else {
                Err(eyre::eyre!("No preimage found"))
            }
        };
        server
            .next_preimage_request(Box::new(get_preimage))
            .expect("Should not error");
    }
}
