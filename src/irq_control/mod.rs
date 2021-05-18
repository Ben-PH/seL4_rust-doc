#![allow(unused_variables, dead_code)]
use crate::kernel_api::objects::capability_space::CapNode;
use crate::types::*;
struct IRQControl {}

impl IRQControl {
    fn create_handler(&self, irq: Word, root: &mut CapNode, idx: CapPtr, depth: u8) -> () {}
}
struct IRQHandler {}
impl IRQHandler {
    fn ack(&self) {}
    fn clear(handler: IRQHandler) {}
    fn set_notification(&mut self, ntfn: CapPtr) {}
}
