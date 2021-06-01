//! The kernel object that Threads use to send/recieve IPC messages.
#![allow(unused_variables, dead_code)]
use crate::kernel_api::syscalls::{SeL4Recv, SeL4Send, SeL4SendRecv };
use crate::types::{capabilities::EndPoint, Badge, CapPtr, MessageInfo};

/// Allow data and caps (namely: IPCBuffer) to x-fered between 2 threads.
///
/// Invoked directly using systemcalls
impl EndPoint {
    /// adds a capability to be included in the next message send
    /// Requires Grand permissions on this endpoint
    /// * Without Grant permissions, only raw msg sent (no cap x-fer)
    pub fn load_cap(cap: CapPtr) {
        unimplemented!()
    }
}

impl SeL4Send for EndPoint {
    fn send(&self, msg: MessageInfo) {
        todo!()
    }
    fn nb_send(&self, msg: MessageInfo) {
        todo!()
    }
}
impl SeL4Recv for EndPoint {
    fn recv(&self, _: bool) -> (MessageInfo, std::option::Option<Badge>) {
        todo!()
    }
    fn nb_recv(&self, get_badge: bool) -> (MessageInfo, std::option::Option<Badge>) {
        todo!()
    }
    fn reply_recv(&self, msg: MessageInfo, _: bool) -> (MessageInfo, std::option::Option<Badge>) {
        todo!()
    }
}

impl SeL4SendRecv for EndPoint {
    fn call(&self, msg: MessageInfo) -> crate::kernel_api::syscalls::SysResult {
        todo!()
    }
}

struct CapBadgeBuf([usize; 120]);
pub struct IPCBuffer {
    tag: MessageInfo,
    /// Content
    msg: [usize; 120],
    /// Base address of structure.
    /// Used by supporting libraries
    user_data: usize,
    /// Buffer for sending caps, and recieving badges.
    caps_badges: CapBadgeBuf,
    /// A CNode to find the recieve slot
    recv_idx: CapPtr,
    /// number of bits recv_indx is to use
    recv_depth: usize,
}
