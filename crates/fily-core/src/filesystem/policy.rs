/// Defines what Fily should do when an operation
/// encounters an existing destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionPolicy {
    /// Stop the operation and report an error.
    Fail,

    /// Replace the existing destination.
    Overwrite,

    /// Leave the existing destination unchanged.
    Skip,
}

impl Default for CollisionPolicy {
    fn default() -> Self {
        Self::Fail
    }
}