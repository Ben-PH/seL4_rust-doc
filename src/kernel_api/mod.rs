//! # A small set of services and objects
//!
//! The microkernel provides a minimal set of service primitives, and a "root-server" application. A "hello-world" root-server is somewhat analagous to a monolithic kernels main function printing "hello-world". The key difference being, that where OS161 and other monolithic-kernel operating systems typically manage OS services and resources from within the kernel, seL4 allows this to be done from the root-server, and other delegated processes, while in user-space.
//!
//! This is done by using these primitives to build and configure OS services as applications running in user-mode. In this way, systems with a rich set of features and extensive complexity  can be built for a broad spectrum of application domains, including full-featured operating systems, without making any changes to the supporting formally verified micro-kernel.
//!
//! These service primitives are made available through capabilities to kernel [objects] that reside within the kernel. Through these capabilities, a user-level process can direct the kernel to carry out instructions that require privileged mode execution using an seL4 system call. This "capability to kernel object via system call" relationship has elements that are comparable to the "File Descriptor to in-kernel virtual file node via system call" relationship.
//!
//! # Capability based access control
//!
//! A capability is an unforgeable token that references a specific instance of kernel-managed objects, and the methods that may be invoked with each object. This governs all kernel services: In order to do something, an application must provide the corresponding token of permission to do it.
//!
//! With this model, a software component can run within a larger system under a high degree of confidence. If it posseses a limited set of allow-listed operation, such as only being able to perform a IPC read through a specific channel, it will be unable to exceed this scope of behaviour due to issues such as bugs and exploints.
//!
//! A [capability_space] is intended to be implemented as a directed graph of [CapNode] kernel-objects, in a way that closely resembles a _guarded page table_ implementation. Refer to module level documentation for more details.
//!
//! # System Calls
//!
//! These provide a message-passing service for communication from [EndPoint]-to-EndPoint, and with
//! kernel objects services. The details of which are documented in the [syscalls] module
//!
//! Logically, the kernel provides three system calls, [Syscall::send], [Syscall::recv] and [Syscall::yield_]. However,
//! there are also combinations and variants of the basic Send and Receive calls. See [syscalls] for more details.
//!
//! # Kernel Memory Allocation
//!
//! The seL4 kernel does not dynamically allocate memory for kernel objects. Instead, objecst are created at the application-level using [UntypedMemory] capabilities that provide the authority to do so. Once created, an object consumes a fixed amount of memory. These mechanisms can precisely control the specific amount of physical memory available to an application or device.
//!
//! At boot time, the kernel pre-allocates the memory it requires for itself. It then spawns a _root server_, to which it hands all remaining memory in the form of a single [UntypedMemory] capability, and a number of other capabilities that form a "primordial soup" of capabilities. From this, you can split the untyped region into smaller regions. The smaller regiouns can be retyped into other kernel objects. It can then delegate all, or part of the resulting authorities to one or more of its clients.
//!
//! # Capability derivation and Memory Reuse
//!
//! An [UntypedMemory] object can be reused under specific circumstances. Refer to module level documentation for more details on the _capability derivation tree_, and the use of `revoke`.

pub mod objects;
pub mod syscalls;

#[cfg(doc)]
use crate::types::capabilities::*;
#[cfg(doc)]
use crate::types::CapPtr;
#[cfg(doc)]
use objects::capability_space;
#[cfg(doc)]
use objects::capability_space::{CapNode, CapSpace};
#[cfg(doc)]
use objects::thread_control_block::ThreadControlBlock;
#[cfg(doc)]
use syscalls::*;
