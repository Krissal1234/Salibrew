mod commands;
use clap::{ Args, Parser, Subcommand};
use commands::todo::TodoCommands;

#[derive(Debug, Parser)]
#[command(name="sali", version, about, long_about = None)]
// #[clap(name = "salibrew", version)]
pub struct Sali{
    // #[clap(flatten)]
    // global_opts: GlobalOpts,
    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Todo(TodoCommands), 
    #[command(arg_required_else_help = true)]
    Test {
        num: i32 
    }
}

#[derive(Debug, Args)]
struct GlobalOpts {
    //... other global options
}

fn main() {
    let cli = Sali::parse();

    match &cli.command {
        Some(Commands::Todo(todo_commands)) =>
        {
            if let Some(description) = &todo_commands.add {
                println!("Adding todo: {}", description)
            }else if let Some(identifier) = &todo_commands.remove {
                println!("Removing todo: {}", identifier);
            }else if todo_commands.list {
                println!("Listing todo's");
            }else if todo_commands.clear{
                println!("Clearing list...");
            }
            else{
            
                println!("No todo specified");
            }
            // dbg!(cli);
        }
        Some(Commands::Test { num }) =>
        {
            println!("test")
        }
        None => {println!("None")}
    }
    // match &cli.command {
    //     Some(Commands::Test {num})=> {
    //         println!("{}",num);
    //     }
    //     None => {println!("None found")}
    // }

}
