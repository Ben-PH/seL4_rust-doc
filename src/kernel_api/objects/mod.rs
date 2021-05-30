//! Implement OS services with the creation, manipulation and combination of these.
//!

pub mod interrupts;
pub mod capability_space;
pub mod endpoints;
pub mod notifications;
pub mod thread_control_block;
pub mod untyped_memory;
// pub mod virtual_address_spaces;
pub mod vspace;
/// These objects represent the set of service primitives provided by the kernel
///
/// TODO Think about where in the library these should appear.
///
/// These service primitives form the building blocks for processes running in
/// user-mode. With an appropriate combination and configuration of service
/// primitives, a user-level process can provide one (or more) services that
/// make up an operating system as a whole.
///
/// # Timer driver example
///
/// TODO outline how a timer driver runs with a TCB, has an EP to which the
/// clock-generated interpts get sent to. Etc.

pub enum Objects {
    CNode,
    ThreadControlBlock,
    EndPoint,
    Notification,
    VirtualAddressSpace,
    Interrupt,
    UntypedMemory,
}
