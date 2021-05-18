pub mod capabilities;
mod err;
mod message;
mod primitives;

pub use primitives::{Badge, Word};

pub use err::Sel4Err;

pub use message::MessageInfo;

pub struct CapPtr(Word);
