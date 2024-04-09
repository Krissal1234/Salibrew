use std::{fs, io::{self, ErrorKind}, path::Path};

use serde::{Serialize,Deserialize};
use clap::Args;

use crate::commands::todo;
#[derive(Debug,Args)]
pub struct TodoCommands{
    #[arg(short,long)]
    pub add: Option<String>,
    #[arg(short,long)]
    pub remove: Option<String>,
    #[arg(short,long)]
    pub list: bool,
    #[arg(short,long)]
    pub clear: bool,
//ideas: Scan file for TODO comments and add to list
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: usize,
    pub description: String,
    pub completed: bool, 
}

impl TodoItem {

    fn load_todos() -> Result<Vec<TodoItem>,io::Error> {
        let file_path = "../data_stores/todo.json";
        if Path::new(file_path).exists() {
            let data = fs::read_to_string(file_path)?;
            serde_json::from_str(&data)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e.to_string()))
        } else {
            Err(io::Error::new(ErrorKind::NotFound, "Todo file not found"))
        }
    }

    pub fn add_new( description: &str) -> Result<(),io::Error> {
    let mut todos = match TodoItem::load_todos() {
        Ok(data) => data,
        Err(e) => return Err(e),
    };
    todos.push(TodoItem {
        id: 2,
        description: description.to_string(),
        completed: false,
    });

    
        Ok(())
    }
}


