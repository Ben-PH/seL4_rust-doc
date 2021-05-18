use crate::types::Word;
pub mod interrupts;

#[cfg(doc)]
use crate::types::Badge;
/// A simple, non-blocking signalling mechanism that logically represents a set of binary semaphores.
///
/// Can be [`Badge`]ed on creation with.
/// TODO: maybe make Badge a trait, and implement it for Ntfn and endpoint?
pub struct Notification {
    pub word: Word,
}

impl Notification {
    /// Has the effect of concurrently signalling multiple spmaphores indicated by its badge
    /// If its badge is unset, or set to 0, it signals just the first thread waiting on the
    /// notification.
    pub fn signal(&mut self) {
        unimplemented!()
    }

    /// if word == 0, then blocks
    /// else, immediate return with previous value
    pub fn wait(self) -> Self {
        unimplemented!()
    }

    /// non-blocking version of wait
    pub fn poll(self) -> Self {
        unimplemented!()
    }
}
