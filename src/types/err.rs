pub use crate::kernel_api::objects::capability_space::CapErr;
pub type Sel4Err<T> = Result<T, CapErr>;
