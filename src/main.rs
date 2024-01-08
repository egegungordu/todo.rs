use todo::todo::{todo::Todo, handler::json_handler::JsonSerializer, task::Task};

fn main() {
    let mut todo = Todo::new(
        Box::new(JsonSerializer::new("todo.json"))
    ).expect("Failed to initialize Todo");

    todo.add_task(Task {
        description: "Hello World".to_string(),
        done: false,
    }).expect("Failed to add task");

    println!("{:?}", todo.get_all_tasks());
}

// todo.from_json(json);
// todo.save_to_disk('path/to/file.json');
// todo.set_done_status(id);
// todo.delete_task(id);
// todo.get_all_tasks();
