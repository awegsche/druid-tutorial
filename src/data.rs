
use std::fs::File;
use std::io::BufReader;

use anyhow::Error;
use druid::{Data, im::Vector};
use druid::{Env, EventCtx, Lens};
use serde::{Deserialize, serde_if_integer128};
use serde::Serialize;
 
#[derive(Debug, Clone, Data, Lens)]
pub struct AppState{
    new_todo: String,
    todos: Vector<TodoItem>
}

impl AppState {
    pub fn new(todos: Vec<TodoItem>) -> Self {
        Self {
            new_todo: "".into(),
            todos: Vector::from(todos),
        }
    }
    pub fn add_todo(&mut self) {
        self.todos.push_front(TodoItem::new(&self.new_todo));
        self.new_todo.clear();
        self.save_to_json().unwrap();
    }

    pub fn click_add(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.add_todo();
    }

    pub fn save_to_json(&self) -> Result<(), Error> {
        let todo_vec: Vec<TodoItem> = self.todos.iter().map(|item| item.to_owned()).collect();
        let serialized = serde_json::to_string_pretty(&todo_vec)?;
        std::fs::write("todos.json", serialized)?;
        Ok(())
    }

    pub fn load_from_json() -> Self {
        let file = File::open("todos.json");

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let todos: Vec<TodoItem> = serde_json::from_reader(reader).unwrap_or(vec![]);
                Self {
                    todos: Vector::from(todos),
                    new_todo: String::new(),
                }
            },
            Err(_) => Self {
                todos: Vector::new(),
                new_todo: String::new(),
            }
        }
    }
}

#[derive(Clone, Data, Lens, Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub done: bool,
    text: String,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            done: false,
            text: text.into(),
        }
    }
}