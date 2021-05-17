
#[allow(unused_imports)]
use crate::kernel::objects::endpoints::EndPoint;
/// TODO confirm thet seL4_Word is usize_t
pub struct Word(pub usize);
/// Optionally added to an [EndPoint] capability on creation,
///
/// By adding a `Badge` to an [EndPoint], it gives a recieving
/// endpoint a means to identify the source of a message.
pub struct Badge(pub Word);

