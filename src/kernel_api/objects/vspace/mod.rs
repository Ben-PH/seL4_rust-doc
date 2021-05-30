//! [ASIDPool] and [ASIDControl] for tracking status of an address space.

pub mod arm;
use crate::types::SeL4Error;
use crate::kernel_api::objects::untyped_memory::Untyped;
use crate::kernel_api::CapSpace;
#[cfg(doc)]
use arm::VSpace;

/// Documentation not present for aarch64
pub struct IOSpace;

/// Confers the right to create a subset the available applications
///
/// For a VSpace to be usable by an app, it must be assigned an ASID using the [ASIDPool::assign] fn
pub struct ASIDPool;

impl ASIDPool {
    /// Gives a [VSpace] an id, by way of placing it in an `ASIDPool`
    ///
    /// This method must be run before a vspace can be used.
    pub fn assign(&mut self, vspace: arm::VSpace) -> Result<(), SeL4Error> {
        unimplemented!()
    }
}

/// The capability from which [ASIDPool] capabilities can be created
pub struct ASIDControl;

impl ASIDControl {
    pub fn make_pool(
        &mut self,
        untyped: Untyped,
        cspace: &mut CapSpace,
    ) -> Result<Self, SeL4Error> {
        unimplemented!()
    }
}
