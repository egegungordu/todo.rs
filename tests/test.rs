use rand::Rng;
use todo::todo::{handler::json_handler::JsonSerializer, task::Task, todo::Todo};

mod mock_handler;

use mock_handler::MockHandler;

fn generate_random_string() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

fn remove_file(path: &str) {
    std::fs::remove_file(path).unwrap_or(());
}

#[test]
fn test_add_task() {
    let mut todo = Todo::new(Box::new(MockHandler::new())).expect("Failed to initialize Todo");

    todo.add_task(Task {
        description: "Hello World".to_string(),
        done: false,
    })
    .expect("Failed to add task");

    assert_eq!(todo.get_all_tasks().len(), 1);
}

#[test]
fn test_remove_task() {
    let mut todo = Todo::new(Box::new(MockHandler::new())).expect("Failed to initialize Todo");

    todo.add_task(Task {
        description: "Hello World".to_string(),
        done: false,
    })
    .expect("Failed to add task");

    todo.remove_task(0).expect("Failed to remove task");

    assert_eq!(todo.get_all_tasks().len(), 0);
}

#[test]
fn test_update_task() {
    let mut todo = Todo::new(Box::new(MockHandler::new())).expect("Failed to initialize Todo");

    todo.add_task(Task {
        description: "Hello World".to_string(),
        done: false,
    })
    .expect("Failed to add task");

    todo.update_task(
        0,
        Task {
            description: "Hello World".to_string(),
            done: true,
        },
    )
    .expect("Failed to update task");

    assert_eq!(todo.get_all_tasks()[0].done, true);
}

#[test]
fn test_get_all_tasks() {
    let mut todo = Todo::new(Box::new(MockHandler::new())).expect("Failed to initialize Todo");

    todo.add_task(Task {
        description: "Hello World".to_string(),
        done: false,
    })
    .expect("Failed to add task");

    todo.add_task(Task {
        description: "Hello World".to_string(),
        done: false,
    })
    .expect("Failed to add task");

    assert_eq!(todo.get_all_tasks().len(), 2);
}

#[test]
fn test_json_handler_initialize() {
    let test_file = format!("{}.json", generate_random_string());

    remove_file(&test_file);

    let _ = Todo::new(Box::new(JsonSerializer::new(&test_file)))
        .expect("Failed to initialize Todo");

    assert_eq!(std::path::Path::new(&test_file).exists(), true);

    remove_file(&test_file);
}

#[test]
fn test_json_handler_add_task() {
    let test_file = format!("{}.json", generate_random_string());
    remove_file(&test_file);

    {
        let mut todo = Todo::new(Box::new(JsonSerializer::new(&test_file)))
            .expect("Failed to initialize Todo");

        todo.add_task(Task {
            description: "Hello World".to_string(),
            done: false,
        })
        .expect("Failed to add task");

        assert_eq!(todo.get_all_tasks().len(), 1);
    }

    {
        let todo = Todo::new(Box::new(JsonSerializer::new(&test_file)))
            .expect("Failed to initialize Todo");

        assert_eq!(todo.get_all_tasks().len(), 1);
    }

    remove_file(&test_file);
}

#[test]
fn test_json_handler_remove_task() {
    let test_file = format!("{}.json", generate_random_string());
    remove_file(&test_file);

    {
        let mut todo = Todo::new(Box::new(JsonSerializer::new(&test_file)))
            .expect("Failed to initialize Todo");

        println!("{:?}", todo.get_all_tasks());

        todo.add_task(Task {
            description: "Hello World".to_string(),
            done: false,
        })
        .expect("Failed to add task");

        todo.remove_task(0).expect("Failed to remove task");

        assert_eq!(todo.get_all_tasks().len(), 0);
    }

    {
        let todo = Todo::new(Box::new(JsonSerializer::new(&test_file)))
            .expect("Failed to initialize Todo");

        assert_eq!(todo.get_all_tasks().len(), 0);
    }

    remove_file(&test_file);
}
