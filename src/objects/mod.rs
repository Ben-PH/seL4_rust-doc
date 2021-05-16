
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
