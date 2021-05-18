use crate::types::Word;

/// A bit-packed word that provides meta-information about an IPC
///
/// label:
/// cap_count
/// unwrapped_caps count
/// length
/// TODO: write an example of recieving, and using some of the data in here.
///
/// Not every use of the sel4 system-calls requires direct manipulation of this struct. libsel4 provides many convinience wrappers that handle configuration for common use-cases.
#[derive(Default)]
pub struct MessageInfo {}

impl MessageInfo {
    pub fn label(&self) -> Word {
        unimplemented!()
    }
}
