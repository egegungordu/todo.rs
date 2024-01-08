use std::{error::Error, fs};

use super::TodoSerializeHandler;
use crate::todo::{todo::Todo, task::Task};

pub struct JsonSerializer {
    file_path: String,
}

impl JsonSerializer {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

impl TodoSerializeHandler for JsonSerializer {
    fn initialize(&mut self) -> Result<Vec<Task>, Box<dyn Error>> {
        let contents = fs::read_to_string(&self.file_path)?;

        let todo: Todo = serde_json::from_str(&contents)?;

        Ok(todo.tasks)
    }

    fn save(&self, todo: &Todo) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string_pretty(todo)?;
        fs::write(&self.file_path, serialized)?;

        Ok(())
    }
}

