use crate::kernel_api::{
    CapSpace,
    capability_space::Slot,
    objects::notifications::Notification,
};
pub struct IRQControl{}
impl IRQControl {
    // pub fn trigger(){unimplemented!()}
    // pub fn trigger_core(){unimplemented!()}
}

pub struct IRQHandler{
    ntfn: Option<Notification>
}

impl IRQHandler {
    pub fn get(
        athority: &mut IRQControl,
        irq_num: usize,
        cspace: &mut CapSpace,
        slot: Slot
    ){unimplemented!()}

    pub fn ack(&self){
        // a driver typically polls or waits on self.ntfn after an ack
        unimplemented!()
    }
    pub fn set_notification(&mut self, ntfn: Notification){
        self.ntfn = Some(ntfn);
        unimplemented!()
    }
    pub fn clear(&mut self){
        self.ntfn = None;
        unimplemented!()
    }
}
