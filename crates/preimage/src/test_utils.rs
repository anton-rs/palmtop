use tempdir::TempDir;

/// A directory for test files.
pub const TEST_DIRECTORY: &str = "palmtop__test";

/// A test file to read from.
pub const TEST_READ_FILE: &str = "test_read.txt";

/// A test file to write to.
pub const TEST_WRITE_FILE: &str = "test_write.txt";

/// Initializes the test directory.
///
/// The returned [TempDir] must be held across the given use of the test directory.
/// When the returned [TempDir] is dropped, the test directory will be deleted.
pub fn init() -> TempDir {
    TempDir::new(TEST_DIRECTORY).unwrap()
}
