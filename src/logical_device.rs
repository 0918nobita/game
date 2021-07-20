pub struct LogicalDevice {
    destroy: Box<dyn FnMut()>,
}

impl LogicalDevice {
    pub fn new(destroy: Box<dyn FnMut()>) -> Self {
        trace!("[CREATED] Logical device");
        LogicalDevice { destroy }
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        (self.destroy)();
        trace!("[DESTROYED] Logical device");
    }
}
