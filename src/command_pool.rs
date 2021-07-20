pub struct CommandPool {
    destroy: Box<dyn FnMut()>,
}

impl CommandPool {
    pub fn new(destroy: Box<dyn FnMut()>) -> Self {
        trace!("[CREATED] CommandPool");
        CommandPool { destroy }
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        (self.destroy)();
        trace!("[DESTROYED] CommandPool");
    }
}
