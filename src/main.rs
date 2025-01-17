mod vite;
mod common;
mod nextjs;
mod nest;
mod nuxt;
mod astro;
mod tauri;

use clap::{Parser, Subcommand};
use crate::astro::create_astro_project;
use crate::common::cmd;
use crate::nest::create_nest_project;
use crate::nextjs::create_next_project;
use crate::nuxt::create_nuxt_project;
use crate::tauri::create_tauri_project;
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
    /// create-nest
    CreateNest,
    /// create-nuxt
    CreateNuxt,
    /// create-astro
    CreateAstro,
    /// create-tauri
    CreateTauri,
}

fn main() {
    let cli = Args::parse();

    // 切换utf8，看是否可以修复彩色 - no work!
    let _ = cmd("chcp 65001").unwrap();
    // println!("{res:#?}");

    match cli.command {
        Some(Commands::CreateVite) => {
            // println!("List users here")
            create_vite_project()
        }
        Some(Commands::CreateNext) => {
            create_next_project()
        }
        Some(Commands::CreateNest) => {
            create_nest_project()
        }
        Some(Commands::CreateNuxt) => {
            create_nuxt_project()
        }
        Some(Commands::CreateAstro) => {
            create_astro_project()
        }
        Some(Commands::CreateTauri) => {
            create_tauri_project()
        }
        None => {
            println!("Run with --help to see instructions.")
        }
    }
}