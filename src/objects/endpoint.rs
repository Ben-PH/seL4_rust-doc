/// Allow data and caps (namely: IPCBuffer) to x-fered between 2 threads.
///
/// Invoked directly using systemcalls
pub struct Endpoint {
    
}

impl Endpoint {
    /// adds a capability to be included in the next message send
    /// Requires Grand permissions on this endpoint
    /// * Without Grant permissions, only raw msg sent (no cap x-fer)
    pub fn load_cap(cap: CapPtr){unimplemented!()}
}

impl SystemCall for Endpoint {
    /// blocking untill recieved
    /// error is ignored: kernel not allowed to reply
    /// requires Write permissions
    fn send() -> Result<??, ()>{}
    /// blocking untill recieved
    /// requires Write permissions
    fn call() -> Result<??, ??>{}

    /// block until something to recieve
    /// raises a fault if without read permissions
    fn recv() {}
    /// second half blocks untill something to recv
    fn reply_recv() {}
}

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
