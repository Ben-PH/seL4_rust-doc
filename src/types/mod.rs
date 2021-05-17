mod primitives;
mod err;
mod message;
pub mod capabilities;


pub use primitives::{
    Word,
    Badge,
};

pub use err::{
    Sel4Err,
};

pub use message::{
    MessageInfo,
};

pub struct CapPtr(Word);
