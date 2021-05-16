/// Allow data and caps (namely: IPCBuffer) to x-fered between 2 threads.
///
/// Invoked directly using systemcalls
pub struct Endpoint {
    
}

impl Endpoint {
    /// adds a capability to be included in the next message send
    /// Requires Grand permissions on this endpoint
    /// * Without Grant permissions, only raw msg sent (no cap x-fer)
    pub fn load_cap(cap: CapPtr){unimplemented!()}
}

impl SystemCall for Endpoint {
    /// blocking untill recieved
    /// error is ignored: kernel not allowed to reply
    /// requires Write permissions
    fn send() -> Result<??, ()>{}
    /// blocking untill recieved
    /// requires Write permissions
    fn call() -> Result<??, ??>{}

    /// block until something to recieve
    /// raises a fault if without read permissions
    fn recv() {}
    /// second half blocks untill something to recv
    fn reply_recv() {}
}
