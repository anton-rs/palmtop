use eyre::Result;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use tracing::instrument;

use palmtop_primitives::{Preimage, PreimageKey};

/// ## OracleClient
///
/// The OracleClient trait defines the interface for a client that requests preimages.
pub trait OracleClient {
    /// Requests a preimage from the oracle.
    fn get(&mut self, key: PreimageKey) -> Result<Preimage>;
}

/// Creates a new OracleClientImpl using a file for reading and writing.
pub fn new_file_client(
    read_filepath: &PathBuf,
    write_filepath: &PathBuf,
) -> OracleClientImpl<BufReader<File>, BufWriter<File>> {
    let file_reader = OpenOptions::new().read(true).open(read_filepath).unwrap();
    let reader = BufReader::new(file_reader);
    let file_writer = OpenOptions::new().write(true).open(write_filepath).unwrap();
    let writer = BufWriter::new(file_writer);
    OracleClientImpl::new(reader, writer)
}

/// OracleClientImpl is an implementation of the [OracleClient] trait.
#[derive(Debug)]
pub struct OracleClientImpl<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    reader: Reader,
    writer: Writer,
}

impl<Reader, Writer> OracleClientImpl<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    /// Creates a new [OracleClientImpl] using the given reader and writer.
    pub fn new(reader: Reader, writer: Writer) -> Self {
        Self { reader, writer }
    }

    /// Reads the length prefix of the preimage from the reader.
    fn read_length_prefix(&mut self) -> Result<u64> {
        let mut length_buf = [0u8; 8];
        self.reader.read_exact(&mut length_buf)?;
        Ok(u64::from_be_bytes(length_buf))
    }
}

impl<Reader, Writer> OracleClient for OracleClientImpl<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    #[instrument(
        name = "preimage_request",
        skip(self),
        fields(server = "oracle_client")
    )]
    fn get(&mut self, key: PreimageKey) -> Result<Preimage> {
        self.writer.write_all(&key)?;
        self.writer.flush()?;
        let length = self.read_length_prefix()?;
        let mut payload = vec![0u8; length as usize];
        self.reader.read_exact(&mut payload)?;
        Ok(payload)
    }
}

/// Test utilities for the preimage server.
#[cfg(feature = "test-utils")]
pub mod test_utils {
    use super::*;
    pub use crate::test_utils::*;
    use byteorder::{BigEndian, WriteBytesExt};

    /// Creates a new [OracleClientImpl] using a file for reading and writing.
    pub fn create_test_client(path: PathBuf) -> OracleClientImpl<BufReader<File>, BufWriter<File>> {
        let read_file_path = path.join(crate::test_utils::TEST_READ_FILE);
        let mut tmp_read_file = File::create(&read_file_path).unwrap();
        let write_file_path = path.join(crate::test_utils::TEST_WRITE_FILE);
        _ = File::create(&write_file_path).unwrap();

        let preimage = vec![1, 2, 3, 4];
        let length_prefix = preimage.len() as u64;
        tmp_read_file.write_u64::<BigEndian>(length_prefix).unwrap();
        tmp_read_file.write_all(&preimage).unwrap();
        tmp_read_file.flush().unwrap();
        new_file_client(&read_file_path, &write_file_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{BigEndian, WriteBytesExt};
    use std::io::Cursor;

    #[test]
    fn test_length_prefix() {
        let mut wtr = vec![];
        let len = 123;
        wtr.write_u64::<BigEndian>(len as u64).unwrap();
        assert_eq!(wtr, [0, 0, 0, 0, 0, 0, 0, 123]);

        let mut client = OracleClientImpl::<Cursor<Vec<u8>>, Cursor<Vec<u8>>>::new(
            Cursor::new(wtr),
            Cursor::new(vec![]),
        );
        let length = client.read_length_prefix().unwrap();
        assert_eq!(length, 123);
    }

    #[cfg(feature = "test-utils")]
    #[test]
    fn test_file_client() {
        let td = test_utils::init();
        let preimage_key = [1; 32];
        let preimage = vec![1, 2, 3, 4];
        let mut client = test_utils::create_test_client(td.path().to_owned());
        let fetched = client.get(preimage_key).expect("Should not error");
        assert_eq!(fetched, preimage);
    }

    #[test]
    fn test_client() {
        let mut wtr = vec![];
        let rdr_ref = vec![];
        let rdr = Cursor::new(rdr_ref);
        let preimage_key = [1; 32];
        let preimage = vec![1, 2, 3, 4];
        wtr.write_u64::<BigEndian>(preimage.len() as u64).unwrap();
        wtr.write_all(&preimage).unwrap();
        let mut client =
            OracleClientImpl::<Cursor<Vec<u8>>, Cursor<Vec<u8>>>::new(Cursor::new(wtr), rdr);
        let fetched = client.get(preimage_key).expect("Should not error");
        assert_eq!(fetched, preimage);
        assert_eq!(client.writer.position(), 32);
        assert_eq!(client.reader.position(), 12);
    }
}
