use super::*;
//
pub struct CapNode {
    pub guard_bits: u8,
    pub radix: u8,
}
impl CapNode {
    /// Copy a capability, sitting its rights in the process
    /// Optionally: Will mint this new cap with a badge, if provided
    pub fn mint(
        dest_root: &mut CapNode,
        dest_slot: CapPtr,
        dest_depth: u8,
        src_root: &CapNode,
        src_idx: CapPtr,
        src_depth: u8,
        rights: CapRights,
        badge: Option<Word>,
    ) -> Result<(),()> {panic!();}

    /// Move a capability from one slot to another.
    /// Set a src_root to move between cap space
    ///
    /// Can optionally mutate the moved/new cap with a new badge
    pub fn r#move(
        dest_root: &mut CapNode,
        dest_idx: CapPtr,
        dest_depth: Word,
        src_root: Option<&mut CapNode>,
        src_idx: CapPtr,
        src_depth: u8,
        mutation: Option<Badge>,
    ) -> Result<(),()> {
        if let Some(src_root) = src_root {
            assert_ne!(src_root as * const _, dest_root as * const _);
        } else {
            // handle the src and dest clashing
        }
        panic!();
    }
    pub fn rotate(
        fst_root: &mut CapNode,
        fst_idx: CapPtr,
        fst_depth: Word,
        scd_root: Option<&mut CapNode>,
        scd_idx: CapPtr,
        scd_depth: u8,
        thd_root: Option<&mut CapNode>,
        thd_idx: CapPtr,
        thd_depth: u8,
    ) -> Result<(),()> {panic!();}
        // fst => scnd
        // scnd => thd
        // thd => fst
    /// Index into a root CapNode and delete that cap
    pub fn delete(
        root_node: &mut CapNode,
        idx: CapPtr,
        depth: u8,
    ) -> Result<(),()>{panic!();}

    /// Delete all child capabilities of a given capability
    /// TODO: grok the documentation (3.2), and translate it
    pub fn revoke(
        root_node: &mut CapNode,
        idx: CapPtr,
        depth: u8,
    ){}

    /// Save the kernel generated reply capability from the
    /// most recent time the thread was called, placing it
    /// into this CapSpace so it can be used later
    pub fn save_caller(
        root_capnode: &mut CapNode,
        idx: CapPtr,
        depth: u8,
    ) -> Result<(),()> {panic!();}
    /// Allows the reuse of badges by an authority.
    ///
    /// Badged Endpoints only
    ///   -> anything else, will have no effect
    ///
    /// The badged endpoint being looked up
    /// has its list of outstanding send operations
    /// with a matching badge
    pub fn cancel_badged_sends(
        root_node: &CapNode,
        // TODO, restrict this to and endpoint only.
        idx: CapPtr,
        depth: u8
    ) -> Result<(),()> {panic!();}
}
