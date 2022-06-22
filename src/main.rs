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

    fn insert_new(&mut self, key: String) {
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

    fn read() -> Self {
        let mut map: HashMap<String, bool> = HashMap::new();
        let contents = fs::read_to_string("todo.txt").expect("Can not open todo.txt");
        for line in contents.split('\n').filter(|&str| str != "") {
            let key = line.split('-').nth(0).unwrap();
            let value = line.split('-').nth(1).unwrap();
            map.insert(key.to_string(), value.parse::<bool>().unwrap());
        }
        Todo {
            map,
        }
    }
}

fn main() {
    let todo = Todo::read();
    todo.list();
}
