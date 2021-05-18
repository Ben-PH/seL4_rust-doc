pub mod capabilities;
// I'm pretty happy with this
pub mod err;
mod message;
mod primitives;

pub use primitives::{Badge, Word};

pub use err::SeL4Error;

pub use message::MessageInfo;

pub struct CapPtr(Word);
