pub use crate::kernel::objects::capability_space::CapErr;
pub type Sel4Err<T> = Result<T, CapErr>;
