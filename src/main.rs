pub mod todo;
use todo::Todo;
use std::env;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
	static ref TODO: Mutex<Todo> = Mutex::new(
		Todo::init()
	);
}

struct Cli {
	action: String,
	object: Option<String>,
}

impl Cli {
	pub fn new() -> Self {
		Self {
			action: String::new(),
			object: None,
		}
	}

	fn parse_cli(&mut self) {
		let mut args = env::args();
		let action = args.nth(1);
		let object = args.nth(0);
		if let Some(action) = action {
			self.action = action;
		} else {
			println!("Usage: <action> [object]");
			return;
		}
		if let Some(object) = object {
			self.object = Some(object);
		} else {
			self.object = None;
		}
	}

	fn deal_cli(&self) {
		match self.action.as_str() {
			"help" => print_help(),
			"add" => {
				match &self.object {
					Some(object) => TODO.lock().insert_new(object.clone()),
					None => println!("Need an object"),
				}
				TODO.lock().save().expect("Failed to save");
			},
			"delete" => {
				match &self.object {
					Some(object) => TODO.lock().delete(object.clone()),
					None => println!("Need an object"),
				}
				TODO.lock().save().expect("Failed to save");
			},
			"complete" => {
				match &self.object {
					Some(object) => TODO.lock().complete(object.clone()),
					None => println!("Need an object"),
				}
				TODO.lock().save().expect("Failed to save");
			},
			"todo" => {
				match &self.object {
					Some(object) => TODO.lock().todo(object.clone()),
					None => println!("Need an object"),
				}
				TODO.lock().save().expect("Failed to save");
			},
			_ => println!("Not a valid action"),
		}
	}
}

fn print_help() {
	println!("Usage: <action> [command]");
	println!("add <object>: add a new todo item");
	println!("delete <object>: if the specified item exists, delete it");
	println!("complete <object>: mark that the object is done");
	println!("todo <object>: mark that the object is not done");
	println!("list: list all items");
}

fn main() {
    let mut cli = Cli::new();
	cli.parse_cli();
	cli.deal_cli();
}
