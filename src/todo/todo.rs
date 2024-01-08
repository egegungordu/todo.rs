use serde::{Serialize, Deserialize};

use super::{task::Task, handler::TodoSerializeHandler};

use std::boxed::Box;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub(crate) tasks: Vec<Task>,
    #[serde(skip)]
    serialize_handler: Box<dyn TodoSerializeHandler>,
}

impl Todo {
    pub fn new(serialize_handler: Box<dyn TodoSerializeHandler>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut todo = Todo {
            tasks: Vec::new(),
            serialize_handler,
        };

        match todo.serialize_handler.initialize() {
            Ok(tasks) => todo.tasks = tasks,
            Err(_) => todo.serialize_handler.save(&todo)?,
        }

        Ok(todo)
    }

    pub fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), Box<dyn std::error::Error>> {
        self.tasks.push(task);
        self.serialize_handler.save(self)
    }

    pub fn remove_task(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.tasks.remove(index);
        self.serialize_handler.save(self)
    }

    pub fn update_task(&mut self, index: usize, task: Task) -> Result<(), Box<dyn std::error::Error>> {
        self.tasks[index] = task;
        self.serialize_handler.save(self)
    }
}
