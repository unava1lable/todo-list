use std::collections::HashMap;
use std::fs;

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

    fn list(&self) {
        for (key, value) in self.map.iter() {
            println!("{} - {}", key, value);
        }
    }

    fn save(&self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in self.map.iter() {
            let record = format!("{}-{}\n", key, value);
            contents.push_str(record.as_str());
        }
        fs::write("todo.txt", contents)?;

        Ok(())
    }
}

fn main() {
    let mut todo = Todo::new();
    todo.insert("Exam".to_string());
    todo.list();
    todo.save();
}
