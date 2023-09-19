use eyre::Result;

/// Hint is an interface that enables any program type to function as a hint,
/// when passed to the Hinter interface, returning a string representation
/// of what data the host should prepare pre-images for.
pub trait Hint {
    /// Returns the hint.
    fn hint(&self) -> String;
}

/// Hinter is an interface to write hints to the host.
/// This may be implemented as a no-op or logging hinter
/// if the program is executing in a read-only environment
/// where the host is expected to have all pre-images ready.
pub trait Hinter {
    /// Hint the pre-image oracle service with the given hint.
    fn hint(&mut self, hint: impl Hint) -> Result<()>;
}
