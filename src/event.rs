#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Event {
    Draw,
    KeyPress(i32),
    EmptyTick,
}

#[derive(Debug, Clone)]
pub struct EventSink {
    pub que: Vec<Event>,
}

impl EventSink {
    /// # push an event into the sink
    pub fn push(&mut self, val: Event) {
        self.que.push(val);
    }

    /// # pops an event from the sink
    pub fn next_q(&mut self) -> Option<Event> {
        self.que.pop()
    }
}
