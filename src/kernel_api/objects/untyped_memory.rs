//!

use crate::kernel_api::objects::capability_space::{CapNode, CapSpace, Slot };
use crate::syscalls::SysResult;
use crate::types::Word;

pub struct Untyped;

impl Untyped {
    pub fn retype(
        &mut self,
        desired_type: Word,
        size_bits: Word,
        cap_space: CapSpace,
        cnode: Slot,
        cnode_offset: Word,
        count: Word,
    ) -> SysResult {
        unimplemented!()
    }
}
