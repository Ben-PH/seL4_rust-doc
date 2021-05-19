//!

use crate::kernel_api::objects::capability_space::CapNode;
use crate::kernel_api::objects::capability_space::CapSpace;
use crate::syscalls::SysResult;
use crate::types::Word;

pub struct Untyped;

impl Untyped {
    pub fn retype(
        self,
        desired_type: Word,
        size: Word,
        cap_space: CapSpace,
        node: CapNode,
        node_offset: Word,
        count: Word,
    ) -> SysResult {
        unimplemented!()
    }
}
