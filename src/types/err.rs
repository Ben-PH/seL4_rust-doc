use crate::types::CapPtr;
#[cfg(doc)]
use crate::types::MessageInfo;
use crate::types::Word;
#[cfg(doc)]
use core::convert::TryFrom;
/// An invalid number used in a system call
pub type InvalidNum = Word;
/// The capability used with a system call in an invalid manner
pub type InvalidCapPtr = CapPtr;
/// Actual memory available
pub type MemoryAvailable = Word;

pub struct RangeErr {
    pub min_allowed: Word,
    pub max_allowed: Word,
}

/// TODO set types
pub struct LookupFailureErr {
    pub for_source_cap: bool,
    pub failure_type: (),
    pub description: (),
}
pub enum SeL4Error {
    /// A non-capability argument is invalid
    InvalidArgument(InvalidNum),
    /// A capability is invoked by invalid means
    InvalidCapability(InvalidCapPtr),
    /// A requested operation is not permitted
    IllegalOperation,
    /// An argument is out of the allowed range
    RangeError(RangeErr),
    /// A supplied argument does not meet the allignment requirements
    AlignmentError,
    /// A capability could not be looked up
    FailedLookup(LookupFailureErr),
    /// Too few message words or capabilities were sent in the message
    TruncatedMessage,
    /// A destination slot specified in the syscall arguments is occupied
    DeleteFirst,
    /// The object currently has other object derived from it
    ///
    /// The requested invocation can be performed when derived objects are deleted, or revoke is invoked on this capability
    RevokeFirst,
    /// Insufficient unallocated space to complete a retype request
    NotEnoughMemory(MemoryAvailable),
}
