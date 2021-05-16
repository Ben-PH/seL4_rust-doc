#![no_std]
//! # WORK IN PROGRESS
//!
//! This library is not even close to being suitable as a reference. Please refer to
//! documentation provided by the seL4 project.
//!
//! # Introduction
//!
//! This library is based on on the seL4 manual, and attribution for the text content,
//! design, etc. goes to this project as a whole. I can only take credit for transcribing
//! my percieved intent of the manual into this form.
//!
//! The seL4 microkernel is an operating-system kernel designed to be a secure, safe, and
//! reliable foundation for systems in a wide variety of application domains. As a microker-
//! nel, it provides a small number of services to applications, such as abstractions to create
//! and manage virtual address spaces, threads, and inter-process communication (IPC).
//! The small number of services provided by seL4 directly translates to a small imple-
//! mentation of approximately 8700 lines of C code. This has allowed the ARMv6 version
//! of the kernel to be formally proven in the Isabelle/HOL theorem prover to adhere to
//! its formal specification [Boy09,CKS08,DEK+06,EKE08,KEH+09,TKN07,WKS+09],
//! which in turn enabled proofs of the kernel’s enforcement of integrity [SWG+11] and
//! confidentiality [MMB+13]. The kernel’s small size was also instrumental in performing
//! a complete and sound analysis of worst-case execution time [BSC+11,BSH12].
//! This library documents the seL4 kernel’s API from a user’s point of view. The document
//! starts by giving a brief overview of the seL4 microkernel design, followed by a reference
//! of the high-level API exposed by the seL4 kernel to userspace.
//!
//! The manual authors tried to ensure their work accurately reflects the behaviour of the
//! seL4 kernel, it is by no means a formal specification of the kernel. When
//! the precise behaviour of the kernel under a particular circumstance needs to be known,
//! users should refer to the seL4 abstract specification, which gives a formal description
//! of the seL4 kernel.
//!
//! # Library author comment
//!
//! This project intends to be a good-faith effort of replicating what the manual aims to
//! communicate, without changing its meaning or detracting from its qualities. It is
//! currently an independent project.
//!
//! # Rust library author comment
//!
//! This project intends to be a good-faith effort of replicating what the manual aims to
//! communicate, without changing its meaning or detracting from its qualities. It is
//! currently an independent project.
//!
//! The intent is to leverage Rusts expressive type system 
//! Idiomatic Rust equivilant might follow these examples
//!
//!  * C "0 on success, error data arrangement depending on error" becomes `Result<(), SomeEnum>`
//!  * When an operation has an "optional extension", and opting into this extension
//!    is achieved by calling a different function with an added argument, these are
//!    can be combined into a single function with an `Option<_>` argument to differentiate.
//!  * Use traits, such as when different kernel objects use a common set of function signatures.
//!
//! I consider the measure of this projects success to be how close its generated docs might
//! be, if the API was originally implemented using Rust while putting significant emphasis on
//! teaching value the documentation provides.

pub mod syscalls;
pub use syscalls::SysCall;
pub mod objects;
pub use objects::Objects;
pub mod cspace;
pub mod types;
pub mod irq_control;

pub enum Service {
    /// Abstraction of CPU execution that supports running software
    Thread,
    /// Virtual memory space
    /// Contains an application
    /// App of one space cannot access memory of another
    AddressSpace,
    /// via endpoints. allows iter-thread message passing
    InterProcessCommunication,
    /// non-blocking signalling mechanism similar to binary semaphores
    Notification,
    /// allows device drivers to be implemented as unprivileged apps.
    /// Kernel exports h/w dev interupts w/ IPC messages
    DevicePrimitives,
    /// Storage for access rights to kern services and their book-keeping info
    CapabilitySpaces,
}

pub mod bibliography {

    //! # Direct copy-paste of the raw text from the manual bibliography section
    //! ```text
    //! [Boy09] Andrew Boyton. A verified shared capability model. In Gerwin Klein, Ralf
    //! Huuck, and Bastian Schlich, editors, Proceedings of the 4th Workshop on
    //! Systems Software Verification, volume 254 of Electronic Notes in Computer
    //! Science, pages 25–44, Aachen, Germany, October 2009. Elsevier.
    //! [BSC+11] Bernard Blackham, Yao Shi, Sudipta Chattopadhyay, Abhik Roychoud-
    //! hury, and Gernot Heiser. Timing analysis of a protected operating system
    //! kernel. In IEEE Real-Time Systems Symposium, pages 339–348, Vienna,
    //! Austria, November 2011.
    //! [BSH12] Bernard Blackham, Yao Shi, and Gernot Heiser. Improving interrupt re-
    //! sponse time in a verifiable protected microkernel. In EuroSys, pages 323–
    //! 336, Bern, Switzerland, April 2012.
    //! [CKS08] David Cock, Gerwin Klein, and Thomas Sewell. Secure microkernels, state
    //! monads and scalable refinement. In Otmane Ait Mohamed, César Muñoz,
    //! and Sofiène Tahar, editors, Proceedings of the 21st International Confer-
    //! ence on Theorem Proving in Higher Order Logics, volume 5170 of Lecture
    //! Notes in Computer Science, pages 167–182, Montreal, Canada, August
    //! 2008. Springer-Verlag.
    //! [DEK+06] Philip Derrin, Kevin Elphinstone, Gerwin Klein, David Cock, and Manuel
    //! M. T. Chakravarty. Running the manual: An approach to high-assurance
    //! microkernel development. In Proceedings of the ACM SIGPLAN Haskell
    //! Workshop, Portland, OR, USA, September 2006.
    //! [EKE08] Dhammika Elkaduwe, Gerwin Klein, and Kevin Elphinstone. Verified pro-
    //! tection model of the seL4 microkernel. In Jim Woodcock and Natarajan
    //! Shankar, editors, Proceedings of Verified Software: Theories, Tools and
    //! Experiments 2008, volume 5295 of Lecture Notes in Computer Science,
    //! pages 99–114, Toronto, Canada, October 2008. Springer-Verlag.
    //! [Int11] Intel Corporation. Intel Virtualization Technology for Directed I/O
    //! — Architecture Specification, February 2011. http://download.intel.com/
    //! technology/computing/vptech/Intel(r)_VT_for_Direct_IO.pdf.
    //! [KEH+09] Gerwin Klein, Kevin Elphinstone, Gernot Heiser, June Andronick, David
    //! Cock, Philip Derrin, Dhammika Elkaduwe, Kai Engelhardt, Rafal Kolan-
    //! ski, Michael Norrish, Thomas Sewell, Harvey Tuch, and Simon Winwood.
    //! seL4: Formal verification of an OS kernel. In Proceedings of the 22nd ACM
    //! 135
    //! 136 BIBLIOGRAPHY
    //! Symposium on Operating Systems Principles, pages 207–220, Big Sky, MT,
    //! USA, October 2009. ACM.
    //! [MMB+13] Toby Murray, Daniel Matichuk, Matthew Brassil, Peter Gammie, Timothy
    //! Bourke, Sean Seefried, Corey Lewis, Xin Gao, and Gerwin Klein. seL4:
    //! from general purpose to a proof of information flow enforcement. In IEEE
    //! Symposium on Security & Privacy, pages 415–429, San Francisco, CA,
    //! May 2013.
    //! [Pal09] Ameya Palande. Capability-based secure DMA in seL4. Masters thesis,
    //! Vrije Universiteit, Amsterdam, January 2009.
    //! [SA99] Tom Shanley and Don Anderson. PCI System Architecture. Mindshare,
    //! Inc., 1999.
    //! [SWG+11] Thomas Sewell, Simon Winwood, Peter Gammie, Toby Murray, June An-
    //! dronick, and Gerwin Klein. seL4 enforces integrity. In Marko van Eekelen,
    //! Herman Geuvers, Julien Schmaltz, and Freek Wiedijk, editor, Interactive
    //! Theorem Proving (ITP), pages 325–340, Nijmegen, The Netherlands, Au-
    //! gust 2011.
    //! [TKN07] Harvey Tuch, Gerwin Klein, and Michael Norrish. Types, bytes, and sep-
    //! aration logic. In Martin Hofmann and Matthias Felleisen, editors, Pro-
    //! ceedings of the 34th ACM SIGPLAN-SIGACT Symposium on Principles
    //! of Programming Languages, pages 97–108, Nice, France, January 2007.
    //! ACM.
    //! [WKS+09] Simon Winwood, Gerwin Klein, Thomas Sewell, June Andronick, David
    //! Cock, and Michael Norrish. Mind the gap: A verification framework for
    //! low-level C. In Stefan Berghofer, Tobias Nipkow, Christian Urban, and
    //! Makarius Wenzel, editors, Proceedings of the 22nd International Confer-
    //! ence on Theorem Proving in Higher Order Logics, volume 5674 of Lecture
    //! Notes in Computer Science, pages 500–515, Munich, Germany, August
    //! 2009. Springer-Verlag.
    //! ```
}
