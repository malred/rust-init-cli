use std::cmp::PartialEq;
use std::path::Path;
use crate::common::{ask_git_init, ask_install, copy_dir_all, current_exe_pkg, git_init, install, paint_bold, paint_option, paint_remind, paint_underline_white, paint_user_input, read_line};

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
    // println!("Project name (tauri-app)");
    paint_remind("Project name", "tauri-app");
    let project_name = read_line();
    if project_name.is_empty() {
        println!("{}", paint_user_input("tauri-app"))
    } else { println!("{}", paint_user_input(&project_name)); }

    // identifier com.ts_js-vanilla-js.app

    // language
    // println!("Choose which language to use for your frontend › (input 1,2,3 to select)");
    paint_remind("Choose which language to use for your frontend › ", "");
    println!("\t1. {}", paint_underline_white("TypeScript / JavaScript  (pnpm, yarn, npm, bun)"));
    println!("\t2. {}", paint_option("Rust"));
    println!("\t3. {}", paint_option(".NET"));
    let language = read_line();
    let language = match language.to_lowercase().as_str() {
        "1" => {
            println!("{}", paint_user_input("TypeScript / JavaScript  (pnpm, yarn, npm, bun)"));
            Language::TsJs
        }
        "2" => {
            println!("{}", paint_user_input("Rust"));
            Language::Rust
        }
        "3" => {
            println!("{}", paint_user_input(".NET"));
            Language::DotNET
        }
        _ => {
            println!("{}", paint_user_input("TypeScript / JavaScript  (pnpm, yarn, npm, bun)"));
            Language::TsJs
        }
    };

    let mut template = UITmpl::Vanilla;
    match language {
        Language::TsJs => {
            // UI template
            // println!("Choose your UI template › (input 1,2,3.. to select)");
            println!("{}", paint_bold("Choose your UI template › "));
            println!("\t1. {}", paint_underline_white("Vanilla"));
            println!("\t2. {}", paint_option("Vue"));
            println!("\t3. {}", paint_option("React"));
            println!("\t4. {}", paint_option("Angular"));
            let line = read_line();
            template = match line.to_lowercase().as_str() {
                "1" => {
                    println!("{}", paint_user_input("Vanilla"));
                    UITmpl::Vanilla
                }
                "Vanilla" => {
                    println!("{}", paint_user_input("Vanilla"));
                    UITmpl::Vanilla
                }
                "vanilla" => {
                    println!("{}", paint_user_input("Vanilla"));
                    UITmpl::Vanilla
                }
                "2" => {
                    println!("{}", paint_user_input("Vue"));
                    UITmpl::Vue
                }
                "Vue" => {
                    println!("{}", paint_user_input("Vue"));
                    UITmpl::Vue
                }
                "vue" => {
                    println!("{}", paint_user_input("Vue"));
                    UITmpl::Vue
                }
                "3" => {
                    println!("{}", paint_user_input("React"));
                    UITmpl::React
                }
                "React" => {
                    println!("{}", paint_user_input("React"));
                    UITmpl::React
                }
                "react" => {
                    println!("{}", paint_user_input("React"));
                    UITmpl::React
                }
                "r" => {
                    println!("{}", paint_user_input("React"));
                    UITmpl::React
                }
                "4" => {
                    println!("{}", paint_user_input("Angular"));
                    UITmpl::Angular
                }
                "Angular" => {
                    println!("{}", paint_user_input("Angular"));
                    UITmpl::Angular
                }
                "angular" => {
                    println!("{}", paint_user_input("Angular"));
                    UITmpl::Angular
                }
                "a" => {
                    println!("{}", paint_user_input("Angular"));
                    UITmpl::Angular
                }
                _ => {
                    println!("{}", paint_user_input("Vanilla"));
                    UITmpl::Vanilla
                }
            };
        }
        Language::Rust => {
            // UI template
            // println!("Choose your UI template › (input 1,2,3.. to select)");
            println!("{}", paint_bold("Choose your UI template › "));
            println!("\t1. Vanilla - (default)");
            println!("\t2. Yew");
            // println!("\t3. Leptos");
            // println!("\t4. Sycamore");
            let line = read_line();
            template = match line.to_lowercase().as_str() {
                "1" => {
                    println!("{}", paint_user_input("Vanilla"));
                    UITmpl::RustVanilla
                }
                "Vanilla" => {
                    println!("{}", paint_user_input("Vanilla"));
                    UITmpl::RustVanilla
                }
                "vanilla" => {
                    println!("{}", paint_user_input("Vanilla"));
                    UITmpl::RustVanilla
                }
                "2" => {
                    println!("{}", paint_user_input("Yew"));
                    UITmpl::Yew
                }
                "Yew" => {
                    println!("{}", paint_user_input("Yew"));
                    UITmpl::Yew
                }
                "yew" => {
                    println!("{}", paint_user_input("Yew"));
                    UITmpl::Yew
                }
                _ => {
                    println!("{}", paint_user_input("Vanilla"));
                    UITmpl::RustVanilla
                }
            };
        }
        Language::DotNET => {
            // UI template
            println!("{}", paint_bold("Choose your UI template › "));
            println!("\t1. {}", paint_option("Blazor"));
            let line = read_line();
            template = match line.to_lowercase().as_str() {
                "1" => UITmpl::Blazor,
                "Blazor" => UITmpl::Blazor,
                "blazor" => UITmpl::Blazor,
                _ => UITmpl::Blazor
            };
            println!("{}", paint_user_input("Blazor"));
        }
    }

    // ts
    // println!("Choose your UI flavor › ");
    paint_remind("Choose your UI flavor › ", "");
    println!("\t1. {}", paint_underline_white("TypeScript"));
    println!("\t2. {}", paint_option("JavaScript"));
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
    if ts {
        println!("{}", paint_user_input("TypeScript"));
    } else { println!("{}", paint_user_input("JavaScript")); }

    // git
    let git = ask_git_init();
    // install
    let npm_type = ask_install();

    let user_select = UserSelectedTauriApp::new(
        &project_name, language, template, ts,
    );
    // println!("{user_select:?}");

    user_select.init();

    install(&user_select.project_name, &npm_type);
    if git { git_init(&user_select.project_name); }
}

