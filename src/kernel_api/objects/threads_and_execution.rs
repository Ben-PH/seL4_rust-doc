//! Representing an execution context, and its processor time managemement
//!
//! Each TCB has an assosciated root-level [CapNode] that serves as the entry-point for
//! its [capability_space]. Similarly for VSpace. These spaces can be shared with other threads
//!
//! A TCB may also hav an IPC buffer, which is used to pass extra arguments during IPC or jernel object invocation that do not fit in the architecture-defined message registers. Although not compulsory, without one, it won't be able to perform most kernel invocations, due to them requiring cap transfers.
//! Each thread belongs to exactly one [security domain]

#[cfg(doc)]
use super::capability_space::{
    CapNode,
};
#[cfg(doc)]
use super::capability_space;

pub struct ThreadControlBlock;
