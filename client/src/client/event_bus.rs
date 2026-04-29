pub struct EventBus<T> {
    queue: Vec<T>,
}

impl<T> EventBus<T> {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn emit(&mut self, event: T) {
        self.queue.push(event);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.queue.drain(..)
    }
}
