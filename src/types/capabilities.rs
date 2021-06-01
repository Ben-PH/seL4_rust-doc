/// Marker trait to indicate a type-alias on a capability
pub(crate) trait Capability {}
/// A special capability automatically given to reciever of a `Call`
pub struct Reply;
impl Capability for Reply {}
/// Attaches to a [ThreadControlBlock] to become a Send/Recieve "port" for IPCs
///
/// IPC is synchronous: A thread sending/recieving on an endpoint
/// will bloock untill delivery.
///  * This implies delivery only happens when both sender & reciever
///    rendezvous at the endpoint.
///
/// An end-point can be restricted to be send-only/recieve only.
/// An end point can have the grant right: allows sending of capabilities.
pub struct EndPoint;
impl Capability for EndPoint {}
/// A simple signaling mechanism.
///
/// sel4 manual ch5
///
/// Built on an word-size array of flags (aka, binary semaphores)
/// Operations will signal a subset of flags in one operation.
/// Poll to check for flags, blocking until any are signalled.
/// can be signal-only or wait-only.
pub struct Notification;
impl Capability for Notification {}
/// sel4 manual section 2.4
///
/// Foundation of memory allocation.
/// Has a single method that allows creation of new kernel objects
/// On success, calling thread gains capability to newly created object
/// Can be divided into groups of smaller `UntypedMemory` objects.
///  * This allows for delegation of part/all of system memory.
pub struct UntypedMemory;
impl Capability for UntypedMemory {}
/// sel4 manual ch8
///
/// The means by which to recieve and acknowledge hardware interupts.
/// There is initially an `IRQControl` capability, which allows for
/// creation of `IRQHandler` capabilities.
///
/// The `IRQHandler` cap allows for management of a specific interrupt source
/// from a specific device, and is delegated to a device driver accordingly.
/// A thread can wait on, and subsequently acknowledge, individual interupts
pub struct Interrupt;
impl Capability for Interrupt {}
/// A special [Notification] capability to be used in response to an interupt.
///
/// Typically produced by the systems initial thread; one for each possible interupt source. Will then be distributed throughout the system in accordance to need. For example, system initialization would provide the clock interupt handler to a timer-driver.
pub struct IRQHandler;
impl Capability for IRQHandler {}
/// Used to create [IRQHandler] capabilities
///
/// May be used to produce a single IRQHandler capability for each interrupt available in the system.
pub struct IRQControl;
impl Capability for IRQControl {}
/// See Ch6 of seL4 manual
///
/// Represents a thread of execution, with semantics for
/// things like block, unblock, scheduling, etc., depending on
/// applications interaction with other threads
pub struct ThreadControlBlock;
/// See Ch3 of seL4 manual
/// these store capabilities, providing permissions to invoke abject methods
/// Each `CNode` has a fixed number of slots (2^n)
pub struct CapNode;
impl Capability for CapNode {}
/// sel4 manual ch8
///
/// Used to construct a virtual address space for 1 or more threads
///
/// Generally are directly related to hardware, ond so are architecture-dependant.
///
/// Kernel also includes ASID Pool and ASID Control objects for tracking Addr. Space. status
pub struct VirtualAddressSpace;
impl Capability for VirtualAddressSpace {}
