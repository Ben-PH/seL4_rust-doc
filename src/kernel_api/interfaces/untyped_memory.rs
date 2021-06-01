//!

use crate::kernel_api::interfaces::capability_space::{CapSpace, Slot };
use crate::kernel_api::syscalls::SysResult;
pub use crate::types::capabilities::UntypedMemory;
use crate::types::Word;


impl UntypedMemory {
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
