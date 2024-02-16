use chrono::Utc;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub headline: String,
    pub description: String,
    pub is_done: bool,
}

#[derive(Debug)]
pub struct TodoCollection {
    todos: Vec<Todo>,
    persist_file_path: String
}

impl TodoCollection {
    pub fn new(persist_file_path: &str) -> Self {
        TodoCollection {
            todos: TodoCollection::get_existing_todos(&persist_file_path),
            persist_file_path: persist_file_path.to_string()
        }
    }

    pub fn add(&mut self, headline: &str, description: &str) -> Todo {
        let todo = Todo {
            id: TodoCollection::generate_id(),
            headline: headline.to_string(),
            description: description.to_string(),
            is_done: false,
        };
        self.todos.push(todo.clone());
        self.persist();
        todo
    }

    pub fn update(&mut self, todo_id: &str, headline: &str, description: &str, is_done: bool) -> Todo {
        let todo = Todo {
            id: todo_id.to_string(),
            headline: headline.to_string(),
            description: description.to_string(),
            is_done: is_done,
        };
        self.todos.retain(|todo| todo.id != todo_id);
        self.todos.push(todo.clone());
        self.persist();
        todo
    }

    pub fn get(&self) -> Vec<Todo>{
        self.todos.clone()
    }

    pub fn get_todo_by_id(self, id: &str) -> Option<Todo> {
        self.todos.iter().find(|&todo| todo.id == id).cloned()
    }

    pub fn list(&self) {
        println!("== Existing todos ==");
        if self.todos.len() == 0 {
            println!("There are no saved Todos");
        } else {
            for todo in &self.todos {
                println!("[{}] {}", todo.id, todo.headline)
            }
        }
    }

    pub fn remove(&mut self, todo_id: &str) {
        match self.todos.iter().find(|&todo| todo.id == todo_id) {
            Some(todo_to_remove) => {
                println!(
                    "Removed Todo[{}] '{}'",
                    todo_to_remove.id, todo_to_remove.headline
                );
                self.todos.retain(|todo| todo.id != todo_id);
                self.persist();
            }
            None => {
                println!("Todo with id [{}] was not found", todo_id)
            }
        };
    }

    fn get_existing_todos(persist_file_path: &str) -> Vec<Todo> {
        let existing_todos_serialized =
            fs::read_to_string(persist_file_path).unwrap_or_else(|_| "[]".to_string());
        return serde_json::from_str(&existing_todos_serialized).unwrap_or_else(|_| Vec::new());
    }

    fn generate_id() -> String {
        let mut hasher = Sha256::new();
        hasher.update(Utc::now().to_string());
        hex::encode(hasher.finalize())[..6].to_string()
    }
    fn persist(&self) {
        let serialized = serde_json::to_string(&self.todos).expect("Failed to serialize data");
        fs::write(&self.persist_file_path, serialized).expect("Could not write to file");
    }
}
