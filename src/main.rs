use todo::todo::task::Task;
use todo::todo::{handler::json_handler::JsonSerializer, todo::Todo};
use todo::ui::ui::TodoUI;

fn main() {
    let serializer = JsonSerializer::new("todo.json");

    let mut todo = Todo::new(Box::new(serializer)).expect("Failed to initialize Todo");

    todo.add_task(Task {
        description: "Hello World".to_string(),
        done: false,
    }).expect("Failed to add task");

    todo.add_task(Task {
        description: "Hello World very very very long long long long long very long task very sakdjaskdjaksdjaksdasdasdasdasdasdasda asd asd asd asd asd".to_string(),
        done: true,
    }).expect("Failed to add task");

    let mut ui = TodoUI::new(todo);

    ui.run().expect("Failed to run UI");
}
