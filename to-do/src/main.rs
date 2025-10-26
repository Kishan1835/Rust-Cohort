use std::{collections::HashMap, io};
struct List {
    id: u32, // unsigned value as it id can't be -ve
    title: String,
    desc: String,
    status: String, // will be using enums in future as it retricts using fixed values(pending and completed)
    created_at: String,
}
struct Database {
    db: HashMap<u32, List>,
    next_id: u32,
}

impl Database {
    fn new() -> Self {
        Database {
            db: HashMap::new(), // they are the associated function dont have (key self)
            next_id: 1,
        }
    }
    fn take_input(&self) -> String {
        // mut used to make it mutable
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command) // refrence to the mutable varibale
            .expect("Failed to read the input");
        command
    }

    fn get_command(&self, command: String) -> (String, String) {
        println!("command {}", command);
        let data: Vec<&str> = command.split_whitespace().collect();
        (data[0].to_string(), format!("{} {}", data[1], data[2]))
    }

    
}

fn main() {
    println!("\n::Welcome to TODO App::\n");
    println!("Enter Help for commands\n");
    let db = Database::new();
    let input = db.take_input();
    let commands: (String, String) = db.get_command(input);
    println!("commands :: {:?}", commands);
    db.process_commands(&commands)
}
