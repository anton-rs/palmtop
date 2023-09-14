use eyre::Result;

/// PreimageKey is a byte array of fixed length 32.
pub type PreimageKey = [u8; 32];

/// Preimage is a byte vector of arbitrary length.
pub type Preimage = Vec<u8>;

/// PreimageGetter is a function that takes a preimage key and returns a preimage.
pub type PreimageGetter = Box<dyn Fn(PreimageKey) -> Result<Preimage> + Send + Sync>;
