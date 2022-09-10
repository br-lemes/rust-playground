use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (String::from(v[0]), bool::from_str(v[1]).unwrap()))
            .collect();
        Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }

    fn done(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = false;
                Some(())
            }
            None => None,
        }
    }

    fn undo(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = true;
                Some(())
            }
            None => None,
        }
    }

    fn list(&mut self, key: &str) {
        match key {
            "all" => {
                for (key, value) in &self.map {
                    let status = if *value { "Active" } else { "Done" };
                    println!("{} -> {}", key, status);
                }
            }
            _ => match self.map.get_mut(key) {
                Some(v) => {
                    let status = if *v { "Active" } else { "Done" };
                    println!("{} -> {}", key, status);
                }
                None => println!("Item {} not found in the list!", key),
            },
        }
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    let mut todo = Todo::new().expect("Initialization of db failed");
    match action.as_str() {
        "insert" => {
            todo.insert(item);
            match todo.save() {
                Ok(_) => println!("Insert successfully!"),
                Err(why) => println!("An error occurred: {}", why),
            };
        }
        "remove" => {
            todo.remove(item);
            match todo.save() {
                Ok(_) => println!("Remove successfully!"),
                Err(why) => println!("An error occurred: {}", why),
            };
        }
        "done" => match todo.done(&item) {
            None => println!("Item '{}' not found in the list!", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Done successfully!"),
                Err(why) => println!("An error occurred: {}", why),
            },
        },
        "undo" => match todo.undo(&item) {
            None => println!("Item '{}' not found in the list!", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Undone successfully!"),
                Err(why) => println!("An error occurred: {}", why),
            },
        },
        "list" => todo.list(&item),
        &_ => println!("Unknown action: {}", action),
    }
}
