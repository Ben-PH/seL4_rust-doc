pub struct IRQControl{}
impl IRQControl {
    pub fn get(){}
    pub fn trigger(){}
    pub fn trigger_core(){}
}

pub struct IRQHandler{}

impl IRQHandler {
    pub fn ack(){}
    pub fn set_notification(){}
    pub fn clear(){}
}
