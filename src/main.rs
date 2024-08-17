mod vite;
mod common;
mod nextjs;

use clap::{Parser, Subcommand};
use crate::nextjs::create_next_project;
use crate::vite::create_vite_project;

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

// clap可以通过反射宏读取注释
#[derive(Subcommand)]
enum Commands {
    /// create-vite
    CreateVite,
    /// create-next
    CreateNext,
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::CreateVite) => {
            // println!("List users here")
            create_vite_project()
        }
        Some(Commands::CreateNext) => {
            create_next_project()
        }
        None => {
            println!("Run with --help to see instructions.")
        }
    }
}