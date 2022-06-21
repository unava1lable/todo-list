use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, false);
    }
}

fn main() {
    let mut todo = Todo::new();
    todo.insert("Exam".to_string());
    println!("{:?}", todo.map);
}
