#[cfg(doc)]
use crate::kernel_api::syscalls::Syscall;

/// A kernel-managed object that provides the service of capability storage.
pub struct CapNode {
    pub guard_bits: u8,
    pub guard_val: usize,
    pub radix: u8,
}
