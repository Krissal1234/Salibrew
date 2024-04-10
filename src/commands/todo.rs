use std::{fs::{self, File}, io::{self, ErrorKind},path::{Path,PathBuf}};
// use directories::ProjectDirs;
use serde::{Serialize,Deserialize};
use clap::Args;
const FILE_PATH: &str = "src/data_stores/todo.json"; 

#[derive(Debug,Args)]
pub struct TodoCommands{
    #[arg(short = 'a',long)]
    pub add: Option<String>,
    #[arg(short='r',long)]
    pub remove: Option<String>,
    #[arg(short ='l',long)]
    pub list: bool,
    #[arg(short = 'c',long)]
    pub clear: bool,
    // #[arg(short ,long)]
    // // pub scan: PathBuf, 
    // // #[arg(short = 'p',long)]
    // // pub priority: u32
//ideas: Scan file for TODO comments and add to list
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: u32,
    pub description: String,
    pub completed: bool, 
}

//attempted to use directories library 
// fn get_data_file_path() -> Result<PathBuf, io::Error> {
// if let Some(proj_dirs) = ProjectDirs::from("com","SalibrewUtils","Salibrew") {
//     let data_dir = proj_dirs.data_dir();
//     fs::create_dir_all(data_dir).expect("Failed to create a data directory");
//     Ok(data_dir.join("todo.json"))
// }else{
//     Err(io::Error::new(io::ErrorKind::NotFound, "Could not determine project directory"))
// }
// }
// fn use_path_as_str(path: &PathBuf) -> Result<(), String> {
//     match path.to_str() {
//         Some(path_str) => {
//             // Use path_str, which is a &str
//             // Your function logic here
//             Ok(())
//         },
//         None => Err("Path contains invalid Unicode characters.".to_string()),
//     }
// }

impl TodoItem {
//look into directories crate to save the json file
    fn load_todos() -> Result<Vec<TodoItem>,io::Error> {
        if Path::new(FILE_PATH).exists() {
            let data = fs::read_to_string(FILE_PATH)?;
            //"[{}]" handles the case when there are no items in the todo list
            if data.trim().is_empty() || data.trim() == "[{}]" {
                Ok(Vec::new())
            }else{
            serde_json::from_str(&data)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e.to_string()))
            }
        } else {
            println!("Creating todo data store...");
            File::create(FILE_PATH)?;
            Ok(Vec::new())
        }
    }

    pub fn add_new( description: &str) -> Result<(),io::Error> {

        let mut todos = match TodoItem::load_todos() {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        let new_id = todos.iter().map(|item| item.id).max().unwrap_or(0) + 1;

        todos.push(TodoItem {
            id: new_id,
            description: description.to_string(),
            completed: false,
        });

        let todos_json = serde_json::to_string(&todos).map_err(|e| io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

        fs::write(FILE_PATH, todos_json)?; 
        Ok(()) 
    }

    pub fn list_todos() -> Result<(), io::Error> {
        let todos = match TodoItem::load_todos() {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        if todos.is_empty(){
            println!("Todo list is empty!");
           return Ok(())
        }

        for todo in &todos{
            println!("{}: {}", todo.id,todo.description);
        }

        Ok(())
    }

    pub fn clear_todos() -> Result<(), io::Error> {
        fs::write(FILE_PATH, "[{}]")?; 
        Ok(()) 
    } 
    pub fn remove_item(id: u32) -> Result<(), io::Error> {

        let mut todos = match TodoItem::load_todos() {
            Ok(data) => data,
            Err(e) => return Err(e),
        };
        if let Some(index) = todos.iter().position(|x| x.id == id) {
            todos.remove(index);
        }

        let todos_json = serde_json::to_string(&todos).map_err(|e| io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

        fs::write(FILE_PATH, todos_json)?; 
        Ok(())
    }


}


