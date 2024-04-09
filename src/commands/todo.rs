use serde::{Serialize,Deserialize};
use clap::Args;
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

// impl TodoItem {
//     pub fn add_new(description: &str) -> Result<()> {

//     }
// }
