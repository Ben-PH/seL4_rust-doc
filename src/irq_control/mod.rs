use crate::types::*;
use crate::cspace::{
    CapPtr,
    CapSpaceNode,
};
struct IRQControl {}

impl IRQControl {
    fn create_handler(
        &self,
        irq: Word,
        root: &mut CapSpaceNode,
        idx: CapPtr,
        depth: u8,
    ) -> () {}
}
struct IRQHandler {}
impl IRQHandler {
    fn ack(&self) {}
    fn clear(handler: IRQHandler) {}
    fn set_notification(&mut self, ntfn: CapPtr) {}
}
