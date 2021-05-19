#[cfg(doc)]
use crate::kernel_api::syscalls::{
    send,
    recv
};

/// A kernel-managed object that provides the service of capability storage.
pub struct CapNode {
    pub guard_bits: u8,
    pub radix: u8,
}
