var searchIndex = JSON.parse('{\
"sel4_rust_doc":{"doc":"WORK IN PROGRESS","t":[0,0,0,0,0,0,13,13,13,13,4,13,13,13,11,11,0,0,11,11,0,0,0,11,11,11,0,0,3,3,3,3,3,12,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,12,11,11,11,11,11,12,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,11,11,11,11,0,11,11,11,11,11,11,11,11,11,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,11,11,11,11,11,11,11,3,3,11,11,11,11,11,11,11,11,11,11,12,11,12,12,12,12,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,3,11,11,11,11,11,11,11,11,3,3,3,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,8,16,13,3,13,8,4,3,3,3,13,3,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,10,11,11,11,11,11,8,6,8,8,10,10,10,10,5,10,10,12,3,3,11,11,11,11,0,0,11,11,11,11,0,11,11,11,11,11,11,8,8,0,13,3,13,13,3,13,6,13,3,13,13,6,13,6,13,4,6,13,13,13,13,4,13,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,12,11,11,11,11,12,12,12,11,11,11,11,12,12,12,12,12,12,11,11,11,11,11,11,12],"n":["acknowledgements","bibliography","kernel_api","object_interfaces","syscalls","types","CNode","EndPoint","Interrupt","Notification","Objects","ThreadControlBlock","UntypedMemory","VirtualAddressSpace","borrow","borrow_mut","capability_space","endpoints","from","into","notifications","reply","thread_control_block","try_from","try_into","type_id","untyped_memory","vspace","CapNode","CapRights","CapSpace","Guard","Slot","bits","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","can_grant","can_grantReply","can_read","can_write","cancel_badged_sends","delete","depth","from","from","from","from","from","idx","into","into","into","into","into","mint","move_","revoke","root_cnode","rotate","save_caller","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","value","EndPoint","borrow","borrow_mut","call","from","into","load_cap","nb_recv","nb_send","recv","reply_recv","send","try_from","try_into","type_id","Notification","borrow","borrow_mut","call","from","interrupts","into","nb_recv","nb_send","recv","reply_recv","send","try_from","try_into","type_id","IRQControl","IRQHandler","ack","borrow","borrow","borrow_mut","borrow_mut","clear","from","from","get","into","into","set_notification","try_from","try_from","try_into","try_into","type_id","type_id","Reply","borrow","borrow_mut","from","into","try_from","try_into","type_id","ThreadControlBlock","UserContext","bind_notification","borrow","borrow","borrow_mut","borrow_mut","configure","from","from","into","into","pc","read_registers","sp","spsr","tpidr_el0","tpidrro_el0","try_from","try_from","try_into","try_into","type_id","type_id","x0","x1","x10","x11","x12","x13","x14","x15","x16","x17","x18","x19","x2","x20","x21","x22","x23","x24","x25","x26","x27","x28","x29","x3","x30","x4","x5","x6","x7","x8","x9","UntypedMemory","borrow","borrow_mut","from","into","retype","try_from","try_into","type_id","ASIDControl","ASIDPool","IOSpace","aarch64","assign","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","into","into","into","make_pool","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","CacheControl","Host","Huge","IOPageTable","Large","Mapping","Page","PageDir","PageTable","PageUpperDir","Small","VSpace","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clean_data","clean_data","clean_data","clean_invalidate_data","clean_invalidate_data","clean_invalidate_data","from","from","from","from","from","from","get_address","into","into","into","into","into","into","invalidate_data","invalidate_data","invalidate_data","map","map","map","map","map","map","map_io","re_map","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","unify_instructions","unify_instructions","unify_instructions","unmap","unmap","unmap","unmap","unmap","unmap","SeL4Recv","SeL4Result","SeL4Send","SeL4SendRecv","call","nb_recv","nb_send","recv","reply","reply_recv","send","0","Badge","CapPtr","borrow","borrow","borrow_mut","borrow_mut","capabilities","err","from","from","into","into","message","try_from","try_from","try_into","try_into","type_id","type_id","Capability","FromUntyped","aarch64_vspace","AlignmentError","AllowedRange","DeleteFirst","DepthMisMatch","DepthMismatchData","FailedLookup","ForSourceCap","GuardMismatch","GuardMismatchData","IllegalOperation","InvalidArgument","InvalidCapPtr","InvalidCapability","InvalidNum","InvalidRoot","LookupFailureDesc","MemoryAvailable","MissingCapability","NotEnoughMemory","RangeError","RevokeFirst","SeL4Error","TruncatedMessage","bits_found","bits_found","bits_left","bits_left","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","from","from","from","from","from","guard_found","into","into","into","into","into","max_allowed","min_allowed","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","IPCBuffer","Info","badges","borrow","borrow","borrow_mut","borrow_mut","caps","caps_unwrapped","extra_caps","from","from","into","into","label","length","msg","recv_depth","recv_idx","tag","try_from","try_from","try_into","try_into","type_id","type_id","user_data"],"q":["sel4_rust_doc","","","sel4_rust_doc::kernel_api","","","sel4_rust_doc::kernel_api::object_interfaces","","","","","","","","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::capability_space","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::endpoints","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::notifications","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::notifications::interrupts","","","","","","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::reply","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::thread_control_block","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::untyped_memory","","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::vspace","","","","","","","","","","","","","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::object_interfaces::vspace::aarch64","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::syscalls","","","","","","","","","","","sel4_rust_doc::kernel_api::types","","","","","","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::types::capabilities","","","sel4_rust_doc::kernel_api::types::err","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","sel4_rust_doc::kernel_api::types::message","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Content of manuals Acknowledgements.","Direct copy-paste of the raw text from the manual …","A small set of services and objects","Implement OS services with the creation, manipulation and …","Raw kernel interface. When invoked with a capability, …","","","","","","These objects represent the set of service primitives …","","","","","","A threads capability management system.","The kernel object that Threads use to send/recieve IPC …","","","A simple, non-blocking signalling mechanism that …","[Reply] is a special, “free” capability that is saved …","Representing execution context, and an applications …","","","","A dynamically sized object from which all other kernel …","[ASIDPool] and [ASIDControl] for tracking status of an …","See Ch3 of seL4 manual these store capabilities, …","Used in configuring capability permissions","A root [CapNode], allowing a [ThreadControlBlock] to …","Bits used to address a specific [CapNode]","Used to index into a [CapSpace], to find a specific entry …","","","","","","","","","","","","","","","","Allows the reuse of badges by an authority.","Removes the capability","","","","","","","","","","","","","Copy a capability, setting its rights in the process","Moves a capability from an occupied slot to an empty slot","Equivilent to [CapSpace::delete] on each capability …","","Two moves in a single, atomic operation","Save the kernel generated reply capability from the most …","","","","","","","","","","","","","","","","","Attaches to a [ThreadControlBlock] to become a …","","","","","","adds a capability to be included in the next message send …","","","","","","","","","A simple signaling mechanism.","","","","","A notification dedicated recieving an interupt.","","","","","","","","","","Required to create [IRQHandler] capabilities using its <code>get</code>…","A special [Notification] capability to be used in …","","","","","","","","","","","","","","","","","","","","","","","","","","","Representing an execution context. Can be scheduled, …","","","","","","","The first step in making a thread ready for activation","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","sel4 manual section 2.4","","","","","","","","","The capability from which [ASIDPool] capabilities can be …","Confers the right to create a subset the available …","Documentation not present for aarch64","Representing the aarch64 address management","Gives a [VSpace] an id, by way of placing it in an …","","","","","","","","","","","","","","","","","","","","","","","","","1GiB","Documentation not present for aarch64","2MiB","For map/unmap of a paging object in/out of its host object","Corresponds to a frame of physical memory that that backs …","Level 2 in the 4-level page-table structure.","Level 3 in the 4-level page-table structure.","Level 1 mapping structure. Is effectively a sparse-array …","1KiB","Level 0 in the 4-level page-table structure.","","","","","","","","","","","","","cleans the data cache range given within this page out to …","","","cleans and invalidates the cache range given within this …","","","","","","","","","Get the physical address of the underlying frame","","","","","","","Invalidates the cache range within the page. The start …","","","","","","","","Takes a VSpace capability, and installs a reference to …","","Changes the permissions of an existing mapping","","","","","","","","","","","","","","","","","","","","","","","","","","","Removes an existing mapping","Syscalls that recieve on a capability","","Syscalls that send on a capability.","Allows send and recieve in a single call, thus improving …","Combine send and recv with key difference","As with <code>recv</code>, but does not block.","Polling send on an endpoint","Recieves message.","Uses reply cap generated by <code>call()</code> to respond to it.","High performance chaining of <code>reply()</code> then <code>recv()</code> with …","Delivers msg through named cap & the app to continue","","Optionally added to an [EndPoint] capability on creation,","","","","","","","","","","","","","","","","","","","Marker trait to indicate a type represents a capability","Not all capabilities can be created from […","sel4 manual ch8","A supplied argument does not meet the allignment …","","A destination slot specified in the syscall arguments is …","When resolving a cap, a CNode was traveresed that:","","A capability could not be looked up","","When resolving a cap, a Cnode was traveresd","","A requested operation is not permitted","A non-capability argument is invalid","The capability used with a system call in an invalid …","A capability is invoked by invalid means","An invalid number used in a system call","The root capability is invaled, e.g. not a CNode cap","Deserialised MessageInfo data following a failed CNode …","Actual memory available","A capability needed for an invocation is not present or …","Insufficient unallocated space to complete a retype …","An argument is out of the allowed range","The object currently has other object derived from it","This deserialises the error data contained in a […","Too few message words or capabilities were sent in the …","The CNode guard-size","Bits of current CNode being traversed resolved","Number of bits in the capability pointer left to decode","Number of bits in the capability pointer left to decode","","","","","","","","","","","","","","","","The actual guard of the CNode","","","","","","","","","","","","","","","","","","","","","","","A representation of the data transferred in a systemcall.","A bit-packed word that provides meta-information about an …","Buffer for receiving badges","","","","","Buffer for sending caps","","","","","","","","","Content","number of bits recv_indx is to use","A CNode to find the recieve slot","","","","","","","","Base address of structure. Used by supporting libraries"],"i":[0,0,0,0,0,0,1,1,1,1,0,1,1,1,1,1,0,0,1,1,0,0,0,1,1,1,0,0,0,0,0,0,0,2,3,2,4,5,6,3,2,4,5,6,5,5,5,5,6,6,4,3,2,4,5,6,4,3,2,4,5,6,6,6,6,6,6,6,3,2,4,5,6,3,2,4,5,6,3,2,4,5,6,2,0,7,7,7,7,7,7,7,7,7,7,7,7,7,7,0,8,8,8,8,0,8,8,8,8,8,8,8,8,8,0,0,9,10,9,10,9,9,10,9,9,10,9,9,10,9,10,9,10,9,0,11,11,11,11,11,11,11,0,0,12,12,13,12,13,12,12,13,12,13,13,12,13,13,13,13,12,13,12,13,12,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,13,0,14,14,14,14,14,14,14,14,0,0,0,0,15,16,17,15,16,17,15,16,17,15,16,17,15,17,16,17,15,16,17,15,16,17,15,0,18,19,0,19,0,0,0,0,0,19,0,20,21,22,23,24,19,20,21,22,23,24,19,25,21,19,25,21,19,20,21,22,23,24,19,19,20,21,22,23,24,19,25,21,19,18,20,22,23,24,19,19,19,20,21,22,23,24,19,20,21,22,23,24,19,20,21,22,23,24,19,25,21,19,18,20,22,23,24,19,0,0,0,0,26,27,28,27,0,27,28,29,0,0,29,30,29,30,0,0,29,30,29,30,0,29,30,29,30,29,30,0,0,0,31,0,31,32,0,31,0,32,0,31,31,0,31,0,32,0,0,32,31,31,31,0,31,33,34,33,34,35,33,34,32,31,35,33,34,32,31,35,33,34,32,31,33,35,33,34,32,31,35,35,35,33,34,32,31,35,33,34,32,31,35,33,34,32,31,0,0,36,37,36,37,36,36,37,37,37,36,37,36,37,37,36,36,36,36,37,36,37,36,37,36,36],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],null,null,[[]],[[]],null,null,null,[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,[[["usize",15]],["sel4result",6]],[[["slot",3]],["sel4result",6]],null,[[]],[[]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[]],[[]],[[["capspace",3],["option",4],["option",4],["caprights",3],["slot",3],["badge",3]],["sel4result",6]],[[["badge",3],["option",4],["capnode",3],["slot",3],["option",4]],["sel4result",6]],[[["slot",3]],["sel4result",6]],null,[[["capnode",3],["slot",3],["option",4]],["sel4result",6]],[[["capnode",3],["slot",3]],["sel4result",6]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,[[]],[[]],[[["info",3]],["info",3]],[[]],[[]],[[["capptr",3]]],[[["option",4],["badge",3]],["info",3]],[[["info",3]]],[[["option",4],["badge",3]],["info",3]],[[["option",4],["info",3],["badge",3]],["info",3]],[[["info",3]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],[[]],[[["info",3]],["info",3]],[[]],null,[[]],[[["option",4],["badge",3]],["info",3]],[[["info",3]]],[[["option",4],["badge",3]],["info",3]],[[["option",4],["info",3],["badge",3]],["info",3]],[[["info",3]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[],["sel4result",6]],[[]],[[]],[[]],[[]],[[],["sel4result",6]],[[]],[[]],[[["capspace",3],["irqcontrol",3],["usize",15],["slot",3]],["sel4result",6]],[[]],[[]],[[["notification",3]],["sel4result",6]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,[[["notification",3]],["sel4result",6]],[[]],[[]],[[]],[[]],[[["guard",3],["capspace",3],["notification",3],["option",4],["vspace",3],["option",4]],["sel4result",6]],[[]],[[]],[[]],[[]],null,[[],[["sel4result",6],["usercontext",3]]],null,null,null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[["usize",15],["nonzerousize",3],["capspace",3],["u8",15],["slot",3]],["sel4result",6]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,null,null,null,[[["vspace",3]],[["sel4error",4],["result",4]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["untypedmemory",3],["capspace",3]],[["result",4],["sel4error",4]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["pagegetaddress",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15],["vmattributes",4]]],[[["usize",15],["iospace",3]]],[[["usize",15],["vmattributes",4]]],[[["usize",15],["vmattributes",4]]],[[["usize",15],["vmattributes",4]]],[[["vspace",3],["usize",15],["caprights",3],["vmattributes",4]]],[[["caprights",3],["iospace",3],["usize",15]]],[[["caprights",3],["vspace",3],["vmattributes",4]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,[[["info",3]],["info",3]],[[["option",4],["badge",3]],["info",3]],[[["info",3]]],[[["option",4],["badge",3]],["info",3]],[[["info",3]]],[[["option",4],["info",3],["badge",3]],["info",3]],[[["info",3]]],null,null,null,[[]],[[]],[[]],[[]],null,null,[[]],[[]],[[]],[[]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[]],[[]],[[]],[[]],[[]],null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,[[]],[[]],[[]],[[]],null,null,null,[[]],[[]],[[]],[[]],null,null,null,null,null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null],"p":[[4,"Objects"],[3,"Guard"],[3,"CapNode"],[3,"Slot"],[3,"CapRights"],[3,"CapSpace"],[3,"EndPoint"],[3,"Notification"],[3,"IRQHandler"],[3,"IRQControl"],[3,"Reply"],[3,"ThreadControlBlock"],[3,"UserContext"],[3,"UntypedMemory"],[3,"ASIDPool"],[3,"IOSpace"],[3,"ASIDControl"],[8,"Mapping"],[4,"Page"],[3,"IOPageTable"],[3,"VSpace"],[3,"PageUpperDir"],[3,"PageDir"],[3,"PageTable"],[8,"CacheControl"],[8,"SeL4SendRecv"],[8,"SeL4Recv"],[8,"SeL4Send"],[3,"Badge"],[3,"CapPtr"],[4,"SeL4Error"],[4,"LookupFailureDesc"],[3,"GuardMismatchData"],[3,"DepthMismatchData"],[3,"AllowedRange"],[3,"IPCBuffer"],[3,"Info"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};