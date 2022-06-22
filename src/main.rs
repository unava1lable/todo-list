pub mod todo;

use todo::Todo;

fn main() {
    let mut todo = Todo::init();
    todo.delete("Coding".to_string());
    todo.delete("Coding".to_string());
    todo.save();
}
