use todo::todo::{handler::TodoSerializeHandler, todo::Todo, task::Task};
use std::error::Error;

pub struct MockHandler {}

impl MockHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl TodoSerializeHandler for MockHandler {
    fn initialize(&mut self) -> Result<Vec<Task>, Box<dyn Error>> {
        let tasks = Vec::new();
        Ok(tasks)
    }

    fn save(&self, _: &Todo) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
