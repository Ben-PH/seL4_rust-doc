//! aarch64 stuff

pub struct ASIDControl;
/// a 4k object, that can contain up to 1024 `VSpace`s
pub struct ASIDPool;
pub struct VMAttributes;
pub struct PageGetAddress {
    err: usize,
    paddr: usize,
}

use crate::kernel_api::{
    objects::untyped_memory::Untyped,
    objects::vspace::{ IOSpace, VSpace },
    CapSpace,
    capability_space::{CapRights, Slot },
};

impl ASIDPool {
    /// Uses an `ASIDControl` and an `Untyped` representing a 4k memory object to make a new `ASIDPool`
    pub fn new(authority: &mut ASIDControl, untyped_4k: Untyped, cspace: CapSpace, slot: Slot) {
        unimplemented!()
    }
    /// assings this `ASIDPool` to the `VSpace` passed in
    pub fn assign(&mut self, vspace: VSpace) {
        unimplemented!()
    }
}

pub struct IOPageTable;
impl IOPageTable {
    pub fn map(&mut self, iospace: IOSpace, addr: usize) {
        unimplemented!()
    }
    pub fn unmap(self) {
        unimplemented!()
    }
}

pub struct Page;
impl Page {
    /// cleans the data cache range given within this page out to RAM
    pub fn clean_data(&mut self, start_offset: usize, end_offset_excl: usize) {
        unimplemented!()
    }
    /// cleans and invalidates the cache range given within this page and flushes data out to RAM
    pub fn clean_invalidate_data(&mut self, start_offset: usize, end_offset_excl: usize) {
        unimplemented!()
    }
    /// Invalidates the cache range within the page.
    /// The start and end should be alligned to a cache line boundary where possible
    /// If they are not aligned, an additional clean is done to the outer cache lines in full also
    pub fn invalidate_data(&mut self, start_offset: usize, end_offset_excl: usize) {
        unimplemented!()
    }
    /// Get the physical address of the underlying frame
    pub fn get_address(&self) -> PageGetAddress {
        unimplemented!()
    }
    pub fn map_io(&mut self, iospace: IOSpace, rights: CapRights, addr: usize) {
        unimplemented!()
    }
    /// Takes a VSpace capability, and installs a reference to this page in the lowest-level
    /// unmapped paging structure corresponding to vaddr. Fails with `FailedLookup` error if
    /// the required paging structures are not present
    pub fn map(&mut self, vspace: VSpace, vaddr: usize, rights: CapRights, attr: VMAttributes) {
        unimplemented!()
    }

    /// Removes an existing mapping
    pub fn unmap(&self) {
        unimplemented!()
    }

    /// Changes the permissions of an existing mapping
    pub fn re_map(&mut self, vspace: VSpace, rights: CapRights, attr: VMAttributes) {
        unimplemented!()
    }
    pub fn unify_instructions(&mut self, start_offset: usize, end_offset_excl: usize) {
        unimplemented!()
    }
}

pub struct PageTable;
impl PageTable {
    pub fn map(&mut self, vspace: VSpace, vaddr: usize, rights: CapRights, attr: VMAttributes) {
        unimplemented!()
    }
    pub fn unmap(&self) {
        unimplemented!()
    }
}

/// level 2 in the pageing structure
pub struct PageDirectory;
impl PageDirectory {
    /// map a `PageDirectory` (level 2) to an `PageUpperDirectory` (level 1)
    pub fn map(&mut self, pud: PageUpperDirectory, vaddr: usize, attr: VMAttributes) {
        unimplemented!()
    }
    /// Unmaps this object from a level 1 `PageUpperDirectory`
    pub fn unmap(&self) {
        unimplemented!()
    }
}

pub struct PageGlobalDirectory;
impl PageGlobalDirectory {
    pub fn clean_data(&mut self, start_offset: usize, end_offset_excl: usize) {
        unimplemented!()
    }
    pub fn clean_invalidate_data(&mut self, start: usize, end: usize) {
        unimplemented!()
    }
    pub fn invalidate_data(&mut self, start: usize, end: usize) {
        unimplemented!()
    }
    pub fn unify_instruction(&mut self, start: usize, end: usize) {
        unimplemented!()
    }
}
pub struct PageUpperDirectory;
impl PageUpperDirectory {
    pub fn map(&mut self, pgd: PageGlobalDirectory, vaddr: usize, attr: VMAttributes) {
        unimplemented!()
    }
    pub fn unmap(&self) {
        unimplemented!()
    }
}
