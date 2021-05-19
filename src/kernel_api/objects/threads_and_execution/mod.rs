//! Representing an execution context, and its processor time managemement
//!
//! Each TCB has an assosciated root-level [CapNode] that serves as the entry-point for
//! its [capability_space]. Similarly for VSpace. These spaces can be shared with other threads
//!
//! A TCB may also hav an IPC buffer, which is used to pass extra arguments during IPC or jernel object invocation that do not fit in the architecture-defined message registers. Although not compulsory, without one, it won't be able to perform most kernel invocations, due to them requiring cap transfers.
//! Each thread belongs to exactly one [security domain]

#[cfg(doc)]
use super::capability_space;
#[cfg(doc)]
use super::capability_space::CapNode;

pub mod interrupts;
use crate::kernel_api::objects::capability_space::CapSpace;
use crate::kernel_api::objects::capability_space::Guard;
use crate::syscalls::SysResult;
use crate::types::capabilities::Notification;
use crate::types::CapPtr;
use crate::types::Word;

pub struct ThreadControlBlock {}

impl ThreadControlBlock {
    pub fn bind_notification(&mut self, ntfn: Notification) -> SysResult {
        unimplemented!()
    }
    pub fn config_single_stepping(&mut self, bp_num: u16, num_inst: Word) -> SysResult {
        unimplemented!()
    }
    pub fn configure(
        &mut self,
        fault_ep: Notification,
        cap_space: CapSpace,
        guard: Option<Guard>,
        vspace: (),
        buffer: Word,
        buffer_frame: CapPtr,
    ) -> SysResult {
        unimplemented!()
    }
    // TODO
    // pub fn copy_registers
    // pub fn read_registers
    // pub fn write_registers
    // pub fn get_breakpoint
    // pub fn set_breakpoint
    // pub fn unset_breakpoint
    // pub fn set_affinity
    // pub fn set_ipc_buffer
    // pub fn set_max_ctrl_priority
    // pub fn set_priority
    // pub fn set_sched_params
    // pub fn set_space
    // pub fn suspend
    // pub fn resume
    // pub fn unbind_notification

    // Confirm tls base needed for aarch64
    // pub fn set_space
}
