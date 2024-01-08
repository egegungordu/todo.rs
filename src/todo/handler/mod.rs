pub mod json_handler;

use std::error::Error;

use super::{todo::Todo, task::Task};

pub trait TodoSerializeHandler {
    fn initialize(&mut self) -> Result<Vec<Task>, Box<dyn Error>>;
    fn save(&self, todo: &Todo) -> Result<(), Box<dyn Error>>;
}

impl Default for Box<dyn TodoSerializeHandler> {
    fn default() -> Self {
        Box::new(json_handler::JsonSerializer::new("todo.json"))
    }
}
