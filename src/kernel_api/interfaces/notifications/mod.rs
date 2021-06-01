//! A simple, non-blocking signalling mechanism that logically represents a set of binary semaphores.
//!
//! Can be [`Badge`]ed on creation to provide the reciever with an identity.

pub mod interrupts;

#[cfg(doc)]
use crate::types::Badge;

