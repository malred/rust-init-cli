use std::any::Any;
use std::cmp::PartialEq;
use std::path::Path;
use crate::common::{ask_git_init, ask_install, copy_dir_all, current_exe_pkg, git_init, install, read_line};

#[derive(Debug)]
struct UserSelectedTauriApp {
    project_name: String,
    language: Language,
    tmpl: UITmpl,
    ts: bool,
}

impl UserSelectedTauriApp {
    fn new(
        project_name: &str,
        language: Language,
        uitmpl: UITmpl,
        ts: bool,
    ) -> Self {
        let project_name = if project_name.is_empty() {
            "tauri-app"
        } else { project_name };

        UserSelectedTauriApp {
            project_name: project_name.to_string(),
            language,
            tmpl: uitmpl,
            ts,
        }
    }
    // 创建文件
    fn init(&self) {
        let mut path = "public/tauri/".to_string();

        match self.language {
            Language::TsJs => {
                path += "ts_js-";

                match self.tmpl {
                    UITmpl::Vanilla => { path += "vanilla" }
                    UITmpl::Vue => { path += "vue" }
                    UITmpl::React => { path += "react" }
                    UITmpl::Angular => { path += "angular" }
                    _ => {}
                }
                // angular只有ts
                if !path.contains("angular") {
                    if self.ts { path += "-ts" } else { path += "-js" }
                }
            }
            Language::Rust => {
                path += "rust-";

                match self.tmpl {
                    UITmpl::RustVanilla => { path += "vanilla" }
                    UITmpl::Yew => { path += "yew" }
                    _ => {}
                }
            }
            Language::DotNET => { path += "DotNET-blazor" }
        }

        println!("复制: {}", current_exe_pkg() + &path);

        copy_dir_all(
            Path::new(
                &(current_exe_pkg() + &path)
            ),
            Path::new(&self.project_name),
        ).unwrap();
    }
}

// ts/js rust .net
#[derive(Debug, Eq, PartialEq)]
enum Language {
    TsJs,
    Rust,
    DotNET,
}

// vue/react
#[derive(Debug)]
enum UITmpl {
    // ts js
    Vanilla,
    Vue,
    React,
    Angular,
    // rust
    RustVanilla,
    Yew,
    // .net
    Blazor,
}

pub fn create_tauri_project() {
    // project name
    println!("Project name (tauri-app)");
    let project_name = read_line();

    // identifier com.ts_js-vanilla-js.app

    // language
    println!("Choose which language to use for your frontend › (input 1,2,3 to select)");
    println!("\t1. TypeScript / JavaScript  (pnpm, yarn, npm, bun) - (default)");
    println!("\t2. Rust");
    println!("\t3. .NET");
    let language = read_line();
    let language = match language.to_lowercase().as_str() {
        "1" => Language::TsJs,
        "2" => Language::Rust,
        "3" => Language::DotNET,
        _ => Language::TsJs
    };

    let mut template = UITmpl::Vanilla;
    match language {
        Language::TsJs => {
            // UI template
            println!("Choose your UI template › (input 1,2,3.. to select)");
            println!("\t1. Vanilla - (default)");
            println!("\t2. Vue");
            println!("\t3. React");
            println!("\t4. Angular");
            let line = read_line();
            template = match line.to_lowercase().as_str() {
                "1" => UITmpl::Vanilla,
                "Vanilla" => UITmpl::Vanilla,
                "vanilla" => UITmpl::Vanilla,
                "2" => UITmpl::Vue,
                "Vue" => UITmpl::Vue,
                "vue" => UITmpl::Vue,
                "3" => UITmpl::React,
                "React" => UITmpl::React,
                "react" => UITmpl::React,
                "r" => UITmpl::React,
                "4" => UITmpl::Angular,
                "Angular" => UITmpl::Angular,
                "angular" => UITmpl::Angular,
                "a" => UITmpl::Angular,
                _ => UITmpl::Vanilla
            };
        }
        Language::Rust => {
            // UI template
            println!("Choose your UI template › (input 1,2,3.. to select)");
            println!("\t1. Vanilla - (default)");
            println!("\t2. Yew");
            // println!("\t3. Leptos");
            // println!("\t4. Sycamore");
            let line = read_line();
            template = match line.to_lowercase().as_str() {
                "1" => UITmpl::RustVanilla,
                "Vanilla" => UITmpl::RustVanilla,
                "vanilla" => UITmpl::RustVanilla,
                "2" => UITmpl::Yew,
                "Yew" => UITmpl::Yew,
                "yew" => UITmpl::Yew,
                _ => UITmpl::RustVanilla
            };
        }
        Language::DotNET => {
            // UI template
            println!("Choose your UI template › (input 1,2,3.. to select)");
            println!("\t1. Blazor");
            let line = read_line();
            template = match line.to_lowercase().as_str() {
                "1" => UITmpl::Blazor,
                "Blazor" => UITmpl::Blazor,
                "blazor" => UITmpl::Blazor,
                _ => UITmpl::Blazor
            };
        }
    }

    // ts
    println!("Choose your UI flavor › ");
    println!("\t1. TypeScript (default)");
    println!("\t2. JavaScript");
    let ts = read_line();
    let ts = match ts.to_lowercase().as_str() {
        "1" => true,
        "ts" => true,
        "typescript" => true,
        "2" => false,
        "js" => false,
        "javascript" => false,
        _ => true
    };

    // git
    let git = ask_git_init();
    // install
    let npm_type = ask_install();

    let user_select = UserSelectedTauriApp::new(
        &project_name, language, template, ts,
    );
    println!("{user_select:?}");

    user_select.init();

    install(&user_select.project_name, &npm_type);
    if git { git_init(&user_select.project_name); }
}

