//! # Introduction
//!
//! 

pub mod syscalls;
pub use syscalls::SysCall;
pub mod objects;
pub use objects::Objects;
pub mod cspace;
pub mod types;
pub mod irq_control;

pub enum Service {
    /// Abstraction of CPU execution that supports running software
    Thread,
    /// Virtual memory space
    /// Contains an application
    /// App of one space cannot access memory of another
    AddressSpace,
    /// via endpoints. allows iter-thread message passing
    InterProcessCommunication,
    /// non-blocking signalling mechanism similar to binary semaphores
    Notification,
    /// allows device drivers to be implemented as unprivileged apps.
    /// Kernel exports h/w dev interupts w/ IPC messages
    DevicePrimitives,
    /// Storage for access rights to kern services and their book-keeping info
    CapabilitySpaces,
}
