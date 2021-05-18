//! # Capability based access control
//!
//! A capability is an unforgeable token that references a specific instance of kernel-managed objects, and the methods that may be invoked with each object. This governs all kernel services: In order to do something, an application must prove it has permission to do it by providing the appropriate capability along with it.
//!
//! With this model, a software component can run within a larger system under a high degree of confidence. If it posseses a limited set of allow-listed operation, such as only being able to communicate with one other particular application, it will be unable to exceed this scope of behaviour.
//!
//! A [capability_space] used by the [ThreadControlBlock] it resides in by means of a capability to the root [CapNode].
//!
//! # System Calls
//!
//! These provide a message-passing service for communication from [EndPoint]-to-EndPoint, and with
//! kernel objects services. The details of which are documented in the [syscalls] module
//!
//! Logically, the kernel provides three system calls, [send], [recv] and [yield]. However,
//! there are also combinations and variants of the basic Send and Receive calls. See [syscalls] for more details.
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
use objects::threads_and_execution::ThreadControlBlock;
#[cfg(doc)]
use syscalls::*;
