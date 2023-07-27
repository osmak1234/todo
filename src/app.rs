use crate::event::Event;
use crate::event::EventSink;

/// # App state,
/// run = false  => stops exits program
/// ## cursor position (in lists)
/// list = true => cursor in completed tasks
/// item = 3 => 4th task from top is selected
#[derive(Debug, Clone)]
pub struct App {
    pub run: bool,
    pub uncompleted: Vec<Task>,
    pub completed: Vec<Task>,
    /// ## list = true => cursor in completed tasks
    pub list: bool,
    /// ## item = 3 => 4th task from top is selected
    pub item: i32,
    pub event_sink: EventSink,
}

/// To-do item
#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
}

// TODO: load task from memory

impl App {
    pub fn new(event_sink: EventSink) -> Self {
        Self {
            run: true,
            uncompleted: vec![
                Task {
                    name: "Take garbage out".to_string(),
                },
                Task {
                    name: "Take garbage out".to_string(),
                },
            ],
            completed: vec![
                Task {
                    name: "Drink coffee".to_string(),
                },
                Task {
                    name: "Finnlay dispose of the corpse".to_string(),
                },
                Task {
                    name: "Finnlay dispose of the corpse".to_string(),
                },
            ],
            list: false,
            item: 0,
            event_sink,
        }
    }
    pub fn move_cursor(&mut self, up: bool) {
        match (self.list, up) {
            (true, true) => {
                if !self.completed.is_empty() {
                    if self.item == 1 {
                        self.item = (self.completed.len()) as i32;
                    } else {
                        self.item -= 1;
                    }
                }
            }
            (true, false) => {
                if !self.completed.is_empty() {
                    if self.item == self.completed.len() as i32 {
                        self.item = 1;
                    } else {
                        self.item += 1;
                    }
                }
            }
            (false, true) => {
                if !self.uncompleted.is_empty() {
                    if self.item == 0 {
                        self.item = (self.uncompleted.len() - 1) as i32;
                    } else {
                        self.item -= 1;
                    }
                }
            }
            (false, false) => {
                if !self.uncompleted.is_empty() {
                    if self.item + 1 == self.uncompleted.len() as i32 {
                        self.item = 0;
                    } else {
                        self.item += 1;
                    }
                }
            }
        }
        self.event_sink.push(Event::Draw)
    }
    pub fn toggle_list(&mut self) {
        match self.list {
            true => {
                self.list = false;
                self.item = 0;
            }
            false => {
                self.list = true;
                self.item = 1;
            }
        }

        self.event_sink.push(Event::Draw);
    }

    pub fn toggle_task(&mut self) {
        match self.list {
            true => {
                if !self.uncompleted.is_empty() {
                    self.uncompleted
                        .push(self.completed.remove(self.item as usize - 1));
                    self.move_cursor(true)
                }
            }
            false => {
                if !self.completed.is_empty() {
                    self.completed
                        .push(self.uncompleted.remove(self.item as usize));
                    self.move_cursor(true)
                }
            }
        }
    }
}
