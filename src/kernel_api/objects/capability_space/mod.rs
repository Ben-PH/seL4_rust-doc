//! A threads capability management system.
//!
//!
//! A diagram showing an example layout of a cap-space with 32 bits. Legal, though impractical.
//!
//!  * It has circular references
//!  * Small number of slots
//!
//! Implemented with same principals as a _guard page table_
//! ```text
//!                                       Guard
//!                                      ┌──────────────┐
//!                             ┌──────► │ 0x0(4bits)   │
//!       ┌───────────────┐     │        └───────┬──────┘
//!       │ CapNode       │     │                │
//!       └─────────┬─────┘     │        ┌───────┴──────┐
//!                 │           │        │              │
//!                 │           │    0x00├────────────┬─┤
//!                 │           │        │  CapNode   │┼┼───► No bits remaining
//!        Guard    ▼           │        ├────────────┴─┤
//!       ┌──────────────┐      │        │              │
//! ┌───► │ 0x000(12bits)│      │        ├──────────────┤
//! │     └──────┬───────┘      │        │ Object       │
//! │            │              │        ├──────────────┤
//! │     ┌──────┴───────┐      │        │              │
//! │     │              │      │        ├──────────────┤
//! │ 0x00├────────────┬─┤      │        │ Object       │
//! │     │  CapNode   │┼┼──────┘        ├──────────────┤
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
//! │     │ CapNode     │┼───┘                  │
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
//! ```
//!
#![allow(unused_variables, dead_code)]

#[cfg(doc)]
use crate::kernel_api::syscalls::{
    send,
    recv
};
#[cfg(doc)]
use super::threads_and_execution::ThreadControlBlock;
use crate::types::*;
mod cap_node;
pub use cap_node::*;

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
/// Bits used to address a specific CapNode
pub struct Guard {
    pub value: Word,
    pub bits: u8,
}

/// An entry in a [CapNode] that contains a specific capability
/// ```text
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
/// ```
pub struct Slot {
    idx: CapPtr,
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
    bits_found: u8,
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
    GuardMismatch(GuardMismatchData),
}

/// ```text
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
/// ```
pub enum CapRights {
    Read,
    Write,
    Grant,
    GrantReply,
}

/// A root [CapNode], allowing a [ThreadControlBlock] to manage its capabilities
///
/// Similar to how libc API functions are convenience wrappers arround the POSIX api, methods on this object are wrappers, which use the arguments to correctly configure the use of the [send] and [recv] syscalls, or their companions.
pub struct CapSpace {
    root: CapNode,
}

impl CapSpace {
    /// Copy a capability, setting its rights in the process
    ///
    /// Optionally: Will mint this new cap with a badge, if provided. If badge is `None`, then it is the equivilent to `seL4_CNode_Copy`
    fn copy(
        &mut self,
        src_slot: Slot,
        src_root: Option<CapNode>,
        dest_slot: Slot,
        rights: CapRights,
    ) -> Result<(), ()> {
        panic!();
    }
    /// Copy a capability, setting its rights in the process
    ///
    /// Optionally: Will mint this new cap with a badge, if provided
    fn mint(
        &mut self,
        src_slot: Slot,
        src_root: Option<CapNode>,
        dest_slot: Slot,
        rights: CapRights,
        badge: Option<Badge>,
    ) -> Result<(), ()> {
        panic!();
    }

    /// Moves a capability from an occupied slot to an empty slot
    ///
    /// If `mutation` is a value of `Some(_)`, then it is the equivilant of `seL4_CNode_Mutate`
    fn r#move(
        &mut self,
        src_slot: Slot,
        dest_root: Option<&mut CapNode>,
        dest_slot: Slot,
        mutation: Option<Badge>,
    ) -> Result<(), ()> {
        panic!();
    }

    /// Two moves in a single, atomic operation
    ///
    /// The pivot slot must be distinct from the source and destination
    /// The destination slot must be empty, unless it's the same as the source,
    /// in which case, its content will be swapped with the pivot slot
    ///
    /// analagous to the following, only done atomically
    /// ```
    /// // src != dest
    /// move(pivot, src);
    /// move(dest, pivot);
    /// // src == dst
    /// move(src, temp)
    /// move(pivot, dest) // or move(pivot, src), as src == dest
    /// move(temp, pivot)
    /// ```
    fn rotate(
        dest_slot: Slot,
        pivot_root: Option<&mut CapNode>,
        pivot_slot: Slot,
        source_root: Option<&mut CapNode>,
        source_slot: Slot,
    ) -> Result<(), ()> {
        panic!();
    }

    /// Removes the capability
    fn delete(root_node: &mut CapNode, slot: Slot) -> Result<(), ()> {
        panic!();
    }

    /// Equivilent to [delete] on each capability derived from `slot`
    ///
    /// Refer to [Untyped] documentation for further details on
    /// capability derivation.
    fn revoke(&mut self, slot: Slot) {}

    /// Save the kernel generated reply capability from the
    /// most recent time the thread was called, placing it
    /// into this CapSpace so it can be used later
    fn save_caller(root_capnode: &mut CapNode, slot: Slot) -> Result<(), ()> {
        panic!();
    }

    /// Allows the reuse of badges by an authority.
    ///
    /// Badged Endpoints only
    ///   -> anything else, will have no effect
    ///
    /// The badged endpoint being looked up
    /// has its list of outstanding send operations
    /// with a matching badge
    fn cancel_badged_sends(
        root_node: &CapNode,
        // TODO, restrict this to and endpoint only.
        slot: Slot,
    ) -> Result<(), ()> {
        panic!();
    }
}
