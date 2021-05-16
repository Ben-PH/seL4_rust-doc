
struct Notification {
    word: Word,
}

impl Notification {
    /// updates nftn word with bitwise-or
    /// `if let Some(waiting) = waiting_threads.first() { waiting.unblock();}`
    /// if `self` is unbadged, or badged to 0, it matches on ALL waiters
    fn signal(&mut self) {unimplemented!()}

    /// if word == 0, then blocks
    /// else, immediate return with previous value
    fn wait(self) -> Self{
        if self.word == 0 {
            // block
            // return self
        } else {
            // return Self{word: 0};
        }
        unimplemented!()
    }

    /// non-blocking version of wait
    fn poll(self) -> Self {unimplemented!()}
}
