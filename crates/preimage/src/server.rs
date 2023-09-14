use byteorder::{BigEndian, WriteBytesExt};
use eyre::Result;
use std::io::{Read, Write};

pub type PreimageKey = [u8; 32];
pub type Preimage = Vec<u8>;

pub type PreimageGetter = fn(key: PreimageKey) -> Result<Preimage>;

pub trait OracleServer {
    fn next_preimage_request(&mut self, get_preimage: PreimageGetter) -> Result<()>;
}

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
    pub fn new(reader: Reader, writer: Writer) -> Self {
        Self { reader, writer }
    }
}

impl<Reader, Writer> OracleServer for OracleServerImpl<Reader, Writer>
where
    Reader: Read,
    Writer: Write,
{
    fn next_preimage_request(&mut self, get_preimage: PreimageGetter) -> Result<()> {
        // Read the preimage key
        let mut buf = [0; 32];
        self.reader.read_exact(&mut buf)?;

        // Fetch the preimage
        let preimage = get_preimage(buf)?;

        // Write the length prefix
        let mut wtr = vec![];
        wtr.write_u64::<BigEndian>(preimage.len() as u64)?;
        self.writer.write_all(&wtr)?;

        // Write the preimage
        self.writer.write_all(&preimage)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_server() {
        let mut server = OracleServerImpl::new(Cursor::new(vec![]), Cursor::new(vec![]));
        let preimage = vec![1, 2, 3, 4];
        let preimage_key = [1; 32];
        let get_preimage = |key: PreimageKey| -> Result<Preimage> {
            if key == preimage_key {
                Ok(preimage.clone())
            } else {
                Err(eyre::eyre!("No preimage found"))
            }
        };
        server
            .next_preimage_request(get_preimage)
            .expect("Should not error");
    }
}
