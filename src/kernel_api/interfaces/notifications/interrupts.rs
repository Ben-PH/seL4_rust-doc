//! A notification dedicated recieving an interupt.
//!
//! A thread can configure the kernel to signal one interrupt object when an inteript triggers. This allows a thread to wait for interupt to occur by calling [SeL4Recv::recv] on that notification.
//! Configurationo of an interupt is done using an [IRQHandler] capability.
//!
//! The system starts without any IRQHandler capabilities present. The ability to create one is provided through the [IRQControl] capability. This capability can be used to produce a single handler for each interrupt available in the system. Typically, the initial thread of a system will determine which IRQs are required by other components in the system, produce the `IRQHandler` cap for each interupt, then delegate this cap as appropriate.

use crate::kernel_api::interfaces::capability_space::{CapSpace, Slot};
#[cfg(doc)]
use crate::kernel_api::syscalls::SeL4Recv;
use crate::types::capabilities::{IRQControl, IRQHandler, Notification};
impl IRQControl {
    // pub fn trigger(){unimplemented!()}
    // pub fn trigger_core(){unimplemented!()}
}

impl IRQHandler {
    pub fn get(athority: &mut IRQControl, irq_num: usize, cspace: &mut CapSpace, slot: Slot) {
        unimplemented!()
    }

    pub fn ack(&self) {
        // a driver typically polls or waits on self.ntfn after an ack
        unimplemented!()
    }
    pub fn set_notification(&mut self, ntfn: Notification) {
        unimplemented!()
    }
    pub fn clear(&mut self) {
        unimplemented!()
    }
}
