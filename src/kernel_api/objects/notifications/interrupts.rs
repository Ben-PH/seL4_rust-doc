//! A notification dedicated recieving an interupt.
//!
//! A thread can configure the kernel to signal one interrupt object when an inteript triggers. This allows a thread to wait for interupt to occur by calling [Notification::wait] or [Notification::poll] on that notification.
//! Configurationo of an interupt is done using an [IRQHandler] capability.
//!
//! TODO document the IRQHandler cap, and its 3 functions: set_notification, ack, clear
//!
//! The system starts without any IRQHandler capabilities present. The ability to create one is provided through the [IRQControl] capability. This capability can be used to produce a single handler for each interrupt available in the system. Typically, the initial thread of a system will determine which IRQs are required by other components in the system, produce the `IRQHandler` cap for each interupt, then delegate this cap as appropriate.

#[cfg(doc)]
use super::Notification;
#[cfg(doc)]
use crate::types::capabilities::*;
