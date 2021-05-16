pub use primitives::*;
enum Service {
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

mod syscalls;
pub use syscalls::SysCall;
mod objects;
pub use objects::Objects;
pub mod cspace;
pub mod primitives;



/// Implemented as a bit-packed word
struct MessageInfo {
    /// Not intepreted by the kernel
    /// Passed as first data-payload of msg
    /// E.g. to specify the requested operation
    label: Word,
    /// Number of capabilities involved
    extra_caps: Word,
    /// Only used on the recieving side.
    /// Indicates the manner in which caps were recieved
    /// See seL4 manual section 4.2.2
    caps_unwrapped: Word,
    length: Word,
}


struct IPCBuffer {
    tag: MessageInfo,
    /// Content
    msg: [Word ; 120],
    /// Base address of structure.
    /// Used by supporting libraries
    user_data: Word,
    /// Buffer for sending caps, and recieving badges.
    caps_badges: CapBadgeBuf,
    /// A CNode to find the recieve slot
    recv_idx: CapPtr,
    /// number of bits recv_indx is to use
    recv_depth: Word,
}

struct Message {
    /// some message registers
    // registers: [CPURegister ; ??]
    ipc_buffer: IPCBuffer,
}




type Sel4Err<T> = Result<T, CapErr>;
