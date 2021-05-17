//! Implement OS services with the creation, manipulation and combination of these.
//!
//! # Capability based access control
//! 
//! A capability is an unforgeable token that references a specific instance of kernel-managed objects, and the methods that may be invoked with each object. This governs all kernel services: In order to do something, an application must prove it has permission to do it by providing the appropriate capability along with it.
//!
//! With this model, a software component can run within a larger system under a high degree of confidence. If it posseses a limited set of allow-listed operation, such as only being able to communicate with one other particular application, it will be unable to exceed this scope of behaviour.
//!
//! A capability space inhabits a [ThreadControlBlock], its implementation resembling a _guarded page table_. A [CapPtr] is used to indexes into a particular slot of a particular [CapNode], in which a capability may or may not reside. A legal, though not neccisarily practical, example is outlined in the ASCII-flow diagram in the [CapSpace] documentation. See also [CapNode] for further details.
//!
//! # System Calls
//!
//! The seL4 kernel provides a message-passing service for communication between threads.
//! This mechanism is also used for communication with kernel-provided services. There
//! is a standard message format, each message containing a number of data words and
//! possibly some capabilities. The details of which are documented in the [syscalls] module
//!
//! Threads send messages by invoking capabilities within their capability space. When
//! an [EndPoint], [Notification] or [Reply] capability is invoked in this way, the message will be
//! transferred through the kernel to another thread. When other capabilities to kernel
//! objects are invoked, the message will be interpreted as a method invocation in a manner
//! specific to the type of kernel object. For example, a thread of execution can be suspended by invoking a  [ThreadControlBlock]
//! (TCB) capability with the correctly formatted message.
//!
//! Logically, the kernel provides three system calls, [SysCall::send], [SysCall::recv] and [SysCall::yield]. However,
//! there are also combinations and variants of the basic Send and Receive calls. 
//! 4 CHAPTER 2. KERNEL SERVICES AND OBJECTS
//! always targeted at a specific thread instead of going through standard IPC mechanics.
//! Invoking Methods on kernel object other than endpoints and notifications is done with
//! Send or Call, depending on whether or not the invoker wants a reply from the kernel.
//! By using functions provided by the libsel4 API you are guaranteed to always use the
//! most appropriate one. The Yield system call is not associated with any kernel object
//! and is the only operation that does not invoke a capability.
//! 
//! # Kernel Memory Allocation
//!
//! The seL4 kernel does not dynamically allocate memory for kernel objects. Instead, objecst are created at the application-level using [UntypedMemory] capabilities that provide the authority to do so. Once created, an object consumes a fixed amount of memory. These mechanisms can precisely control the specific amount of physical memory available to an application or device.
//!
//! At boot time, the kernel pre-allocates the memory it requires for itself. It then spawns a _root server_, to which it hands all remaining memory in the form of a single [UntypedMemory] capability, and a number of other capabilities that form a "primordial soup" of capabilities. From this, you can split the untyped region into smaller regions. The smaller regiouns can be retyped into other kernel objects. It can all, or part of the resulting authorities to one or more of its clients.
//!
//! # Capability derivation and Memory Reuse
//!
//! An [UntypedMemory] object can be reused under specific circumstances. Refer to module level documentation for more details on the _capability derivation tree_, and the use of `revoke`.

#[cfg(doc)]
use threads_and_execution::ThreadControlBlock;
#[cfg(doc)]
use capability_space::{
    CapNode,
    CapSpace
};
#[cfg(doc)]
use crate::types::capabilities::*;
#[cfg(doc)]
use super::syscalls::SysCall;
#[cfg(doc)]
use super::syscalls;
#[cfg(doc)]
use crate::types::CapPtr;


pub mod capability_space;
pub mod endpoints;
pub mod notifications;
pub mod threads_and_execution;
pub mod untyped_memory;
pub mod virtual_address_spaces;
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
    /// See Ch3 of seL4 manual
    /// these store capabilities, providing permissions to invoke abject methods
    /// Each `CNode` has a fixed number of slots (2^n)
    CNode,

    /// See Ch6 of seL4 manual
    ///
    /// Represents a thread of execution, with semantics for
    /// things like block, unblock, scheduling, etc., depending on
    /// applications interaction with other threads
    ThreadControlBlock,

    /// Send and Recieve "ports" for IPC
    ///
    /// IPC is synchronous: A thread sending/recieving on an endpoint
    /// will bloock untill delivery.
    ///  * This implies delivery only happens when both sender & reciever
    ///    rendezvous at the endpoint.
    ///
    /// An end-point can be restricted to be send-only/recieve only.
    /// An end point can have the grant right: allows sending of capabilities.
    EndPoint,

    /// sel4 manual ch5
    ///
    /// A simple signaling mechanism.
    ///
    /// Built on an word-size array of flags (aka, binary semaphores)
    /// Operations will signal, a subset of flags in one operation.
    /// Poll to check for flags, blocking until any are signalled.
    /// can be signal-only or wait-only.
    Notification,

    /// sel4 manual ch8
    ///
    /// Used to construct a virtual address space for 1 or more threads
    ///
    /// Generally are directly related to hardware, ond so are architecture-dependant.
    ///
    /// Kernel also includes ASID Pool and ASID Control objects for tracking Addr. Space. status
    VirtualAddressSpace,

    /// sel4 manual ch8
    ///
    /// The means by which to recieve and acknowledge hardware interupts.
    /// There is initially an `IRQControl` capability, which allows for
    /// creation of `IRQHandler` capabilities.
    ///
    /// The `IRQHandler` cap allows for management of a specific interrupt source
    /// from a specific device, and is delegated to a device driver accordingly.
    /// A thread can wait on, and subsequently acknowledge, individual interupts
    Interrupt,

    /// sel4 manual section 2.4
    ///
    /// Foundation of memory allocation.
    /// Has a single method that allows creation of new kernel objects
    /// On success, calling thread gains capability to newly created object
    /// Can be divided into groups of smaller `UntypedMemory` objects.
    ///  * This allows for delegation of part/all of system memory.
    UntypedMemory,
}
