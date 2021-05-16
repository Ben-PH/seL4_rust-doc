//! Instances of these are invoked by applications
//!
//! The interfaces to these objects form the interface to the
//! kernel itself. The creation and use of kernel services is
//! achieved by the creation, manipulation, and cmbination of
//! these objects.

//!                                       Guard
//!                                      ┌──────────────┐
//!                             ┌──────► │ 0x0(4bits)   │ 
//!       ┌───────────────┐     │        └───────┬──────┘
//!       │ CNode         │     │                │
//!       └─────────┬─────┘     │        ┌───────┴──────┐
//!                 │           │        │              │
//!                 │           │    0x00├────────────┬─┤
//!                 │           │        │  CNode     │┼┼───► No bits remaining
//!        Guard    ▼           │        ├────────────┴─┤
//!       ┌──────────────┐      │        │              │
//! ┌───► │ 0x000(12bits)│      │        ├──────────────┤
//! │     └──────┬───────┘      │        │ Object       │
//! │            │              │        ├──────────────┤
//! │     ┌──────┴───────┐      │        │              │
//! │     │              │      │        ├──────────────┤
//! │ 0x00├────────────┬─┤      │        │ Object       │
//! │     │  CNode     │┼┼──────┘        ├──────────────┤
//! │     ├────────────┴─┤               │ Object       │
//! │     │              │               ├──────────────┤
//! │     ├──────────────┤               │              │
//! │     │              │               ├──────────────┤
//! │     ├──────────────┤               │              │
//! │     │              │               ├──────────────┤
//! │     ├──────────────┤               │              │
//! │     │              │               ├──────────────┤
//! │     ├──────────────┤               │              │
//! │     │    Object    │           0xFF└──────────────┘
//! │     ├──────────────┤
//! │     │    Object    │                Guard
//! │     ├──────────────┤               ┌──────────────┐
//! │     │              │   ┌──────────►│ o0(3bits)    │
//! │     ├─────────────┬┤   │           └──────┬───────┘
//! │     │ CNode       │┼───┘                  │
//! │     ├─────────────┴┤               ┌──────┴───────┐
//! │     │              │               │              │
//! │ 0xFF└──────────────┘           0x00├──────────────┤
//! │                                    │ Object       │
//! │                                    ├──────────────┤
//! │                                    │ Object       │
//! │                                    ├──────────────┤
//! │                                    │              │
//! │                                    ├──────────────┤
//! │                                    │              │
//! │                                    ├──────────────┤
//! │                                    │ Object       │
//! │                                    ├──────────────┤
//! │                                    │              │
//! │                                    ├─────────────┬┤
//! │                                    │ CNode       │┼────► 5 bits remaining
//! │                                    ├─────────────┴┤
//! │                                    │              │
//! │                                    ├──────────────┤
//! │                                    │              │
//! │                                    ├──────────────┤
//! └────────────────────────────────────┤ CNode        │
//!                                   0xF└──────────────┘
//!
//! Demonstration of a CNode structure.
//!
//! The CNode on the left is our top-level. It has a radix-12 guard
//! set to 0x000, and 256 slots (taking 8 bits).
//! Observe, that with radix-8 slots, max index is 0xFF
//!
//! * a slot can contain an Object, or a reference to another CNode
//! * A slot entry can refer to the top-level CNode
//! * A CNode can have a guard of variable radix
//! * Both 2nd level CNodes have 12 bits remaining
//! * The top Consumes 4(guard) + 8(radix) == 12,
//!   leaving none remaining.
//! * The bottom consumes 3(guard) + 4(radix) bits, leaving 5 bits
//!   for subsequent CNodes.
//!
//! Practically speaking, this example is not much good:
//!  * It has circular references
//!  * Small number of slots

use crate::primitives::*;
pub enum CapErr {
    InvalidArgument,
    InvalidCapability,
    IlligalOperation,
    RangeError,
    AlignmentError,
    FailedLookup,
    TruncatedMessage,
    DeleteFirst,
    RevokeFirst,
    NotEnoughMemoory,
}
pub struct Guard {
    value: Word,
    bits: u8,
}
pub type CapPtr = Word;

pub struct CapSpaceNode {
    guard_bits: u8,
    radix: u8,
}
/// ┌─────────────────────┐
/// │L1 CapNode CapPtr    │
/// └─────────────────────┘
///                 ▼
///       ┌──────────────────┐
/// Guard │ 0x0(4bits)       │
///       └──────────────────┘
/// 
///   0x00┌──────────────────┐
///       │                  │
///   0x0F├──────────────────┤
///       │ L2 CapNode CapPtr├────────────┐
///       ├──────────────────┤            ▼
///       │                  │        ┌──────────────────┐
///       │                  │  Guard │ 0x0(4bits)       │
///       │                  │        └──────────────────┘
///       │                  │
///       │                  │    0x00┌──────────────────┐
///   0x60├──────────────────┤        │ L3 CapNode CapPtr├────────────┐
///       │ CapA             │        ├──────────────────┤            ▼
///       ├──────────────────┤        │                  │        ┌──────────────────┐
///       │                  │        │                  │  Guard │ (0bits)          │
///       │                  │        │                  │        └──────────────────┘
///       │                  │        │                  │
///       │                  │        │                  │        ┌──────────────────┐
///       │                  │        │                  │    0x00│                  │
///   0xFF└──────────────────┘        │                  │        │                  │
///                               0x60├──────────────────┤        │                  │
/// CapA: addressed with 0x0_60xxxxx  │ CapB             │        │                  │
/// CabB: 0x0_0F_0_60_xx              ├──────────────────┤    0x60├──────────────────┤
/// CapC: 0x0_0F_0_00_60              │                  │        │                  │
/// C-G:                              │                  │        │ Cap C, D, E, F, G│
///  base: 0x0_0f_0_00_60             │                  │    0x64├──────────────────┤
///  window: 5                        │                  │        │                  │
///                                   │                  │        │                  │
/// L2 cap itself:                0xFF└──────────────────┘        │                  │
///  * set defth limit: 12bits                                0xFF└──────────────────┘
///  * 0x0_0F_MASKED
///  * with depth limit of 12, only left-most 12 bits are assesed, preventing a dereference
pub struct Slot {
    idx: Word,
    depth: u8,
}

pub type CapLookupResult<T> = Result<T, LookupFailure>;

pub struct GuardMismatchData {
    /// Number of bits in the capability pointer left to decode
    bits_left: u8,
    /// The actual guard of the CNode
    guard_found: u8,
    /// The CNode guard-size
    bits_found: u8,
}
pub struct DepthMismatchData {
    /// Number of bits in the capability pointer left to decode
    bits_left: u8,
    /// Bits of current CNode being traversed resolved
    bits_found:u8
}

pub enum LookupFailure {
    /// The root capability is invaled, e.g. not a CNode cap
    InvalidRoot,
    /// A capability needed for an invocation is not present
    /// or doesn't have sufficient rights.
    /// Provides bits remaining(what does this mean???)
    MissingCapability(u8),

    ///When resolving a cap, a CNode was traveresed that:
    /// * resolved more bits than was left to decode in the cap, or
    /// * a non-CNode capability was encountered with bits remaining
    DepthMisMatch(DepthMismatchData),

    /// When resolving a cap, a Cnode was traveresd
    /// * With a guard-size larger than the # of remaning bits, OR
    /// * The CNode's guard did not match the next bits of cap being resolved
    GuardMismatch(GuardmisMatchData)
}

/// +-------------+-------------+-------------+-------------+-------------+
/// | Type        | Read        | Write       | Grant       | GrantReply  |
/// +-------------+-------------+-------------+-------------+-------------+
/// | Endpoint    | Receiving   | Sending     | Sending     |Sending reply|
/// |             |             |             | capabilities|capabilities |
/// +-------------+-------------+-------------+-------------+-------------+
/// | Notification| Waiting     | Signalling  | N/A         | N/A         |
/// |             |             |             |             |             |
/// +-------------+-------------+-------------+-------------+-------------+
/// | Page        | Mapping page| Mapping page| N/A         | N/A         |
/// |             | readable    |writable     |             |             |
/// +-------------+-------------+-------------+-------------+-------------+
/// | Reply       | N/A         | N/A         | Sending any |             |
/// |             |             |             | capabilities|             |
/// |             |             |             | in reply    |             |
/// |             |             |             | message     |             |
/// +-------------+-------------+-------------+-------------+-------------+
enum CapRights {
    Read,
    Write,
    Grant,
    GrantReply,
}

impl CapSpaceNode {
    /// Copy a capability, sitting its rights in the process
    /// Optionally: Will mint this new cap with a badge, if provided
    fn mint(
        dest_root: &mut CapSpaceNode,
        dest_slot: CapPtr,
        dest_depth: u8,
        src_root: &CapSpaceNode,
        src_idx: CapPtr,
        src_depth: u8,
        rights: CapRights,
        badge: Option<Word>,
    ) -> CapLookupResult<()> {}

    /// Move a capability from one slot to another.
    /// Set a src_root to move between cap space
    ///
    /// Can optionally mutate the moved/new cap with a new badge
    fn r#move(
        dest_root: &mut CapSpaceNode,
        dest_idx: CapPtr,
        dest_depth: Word,
        src_root: Option<&mut CapSpaceNode>,
        src_idx: CapPtr,
        src_depth: u8,
        mutation: Option<Badge>,
    ) -> CapLookupResult<()> {
        if let Some(other_space) = src_root {
            assert_ne!(other_space.as_ptr(), dest_root.as_ptr());
        } else {
            // handle the src and dest clashing
        }
        unimplemented!();
    }
    fn rotate(
        fst_root: &mut CapSpaceNode,
        fst_idx: CapPtr,
        fst_depth: Word,
        scd_root: Option<&mut CapSpaceNode>,
        scd_idx: CapPtr,
        scd_depth: u8,
        thd_root: Option<&mut CapSpaceNode>,
        thd_idx: CapPtr,
        thd_depth: u8,
    ) -> CapLookupResult<()> {}
        // fst => scnd
        // scnd => thd
        // thd => fst
    /// Index into a root CapNode and delete that cap
    fn delete(
        root_node: &mut CapSpaceNode,
        idx: CapPtr,
        depth: u8,
    ) -> CapLookupResult<()>{}

    /// Delete all child capabilities of a given capability
    /// TODO: grok the documentation (3.2), and translate it
    fn revoke(
        root_node: &mut CapSpaceNode,
        idx: CapPtr,
        depth: u8,
    ){}

    /// Save the kernel generated reply capability from the
    /// most recent time the thread was called, placing it
    /// into this CapSpace so it can be used later
    fn save_caller(
        root_capnode: &mut CapSpaceNode,
        idx: CapPtr,
        depth: u8,
    ) -> CapLookupResult<()> {}
    /// Allows the reuse of badges by an authority.
    ///
    /// Badged Endpoints only
    ///   -> anything else, will have no effect
    ///
    /// The badged endpoint being looked up
    /// has its list of outstanding send operations
    /// with a matching badge
    fn cancel_badged_sends(
        root_node: &CapSpaceNode,
        // TODO, restrict this to and endpoint only.
        idx: CapPtr,
        depth: u8
    ) -> CapLookupResult<()> {}
}
