pub use crate::cspace::CapErr;
pub type Sel4Err<T> = Result<T, CapErr>;
