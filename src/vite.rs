use std::path::Path;
use ansi_term::{Color, Style};
use crate::common::{ask_git_init, ask_install, copy_dir_all, current_exe_pkg, git_init, install, paint_bold, paint_option, paint_remind, paint_user_input, paint_white, read_line};

#[derive(Debug)]
enum FrameworkType {
    React,
    Vue,
}

#[derive(Debug)]
enum VariantType {
    Javascript,
    Typescript,
}

#[derive(Debug)]
struct UserSelected {
    framework_type: FrameworkType,
    variant_type: VariantType,
    // file_name: String,
    project_name: String,
}

impl UserSelected {
    fn new(project_name: &str, framework: &str, variant: &str) -> Self {
        let framework_type = match framework {
            "react" => {
                FrameworkType::React
            }
            "vue" => {
                FrameworkType::Vue
            }
            _ => {
                // panic!("No such framework type")
                // default
                FrameworkType::React
            }
        };

        let variant_type = match variant {
            "javascript" => {
                VariantType::Javascript
            }
            "typescript" => {
                VariantType::Typescript
            }
            "js" => {
                VariantType::Javascript
            }
            "ts" => {
                VariantType::Typescript
            }
            _ => {
                // panic!("No such variant type")
                VariantType::Typescript
            }
        };

        let project_name = if project_name.is_empty() {
            "vite-project"
        } else { project_name };

        UserSelected {
            project_name: project_name.to_string(),
            framework_type: framework_type,
            variant_type: variant_type,
            // file_name: "".to_string(),
        }
    }

    // 创建文件
    fn init(&self) {
        // let mut path = "./src/public/".to_string();
        let mut path = "public/vite/".to_string();

        match self.framework_type {
            FrameworkType::React => {
                path += "react";
            }
            FrameworkType::Vue => {
                path += "vue";
            }
        }

        path += "-";

        match self.variant_type {
            VariantType::Javascript => {
                path += "js";
            }
            VariantType::Typescript => {
                path += "ts";
            }
        }

        println!("复制 {}", &(current_exe_pkg() + &path));

        // todo: 从网络上下载 或 调用cmd git clone

        copy_dir_all(
            // src,
            // Path::new(&(current_dir1.to_string() + "/" + &path)),
            Path::new(
                &(current_exe_pkg() + &path)
            ),
            Path::new(&self.project_name),
        ).unwrap();
    }
}

pub fn create_vite_project() {
    // project name
    // println!("{} {}", paint_bold("your project name?"), paint_white("vite-project"));
    paint_remind("your project name?", "vite-project");
    let project_name = read_line();
    let project_name = if project_name.is_empty() {
        "vite-project"
    } else { &project_name };
    println!("{}", paint_user_input(&project_name));

    // select a framework
    // react vue ...
    // println!("select a framework: (default: react)");
    // println!("{} {}", paint_bold("select a framework:"), paint_white("(react)"));
    paint_remind("select a framework:", "(react)");
    // println!("react");
    // println!("vue");
    println!("{}",paint_option("react"));
    println!("{}",paint_option("vue"));
    let mut framework = read_line().to_lowercase();
    framework = match framework.as_str() {
        "r" => { "react".to_string() }
        "react" => { "react".to_string() }
        "v" => { "vue".to_string() }
        "vue" => { "vue".to_string() }
        _ => { "react".to_string() }
    };
    println!("{}", paint_user_input(&framework));

    // select a variant
    // javascript typescript ...
    // println!("select a variant: (default: ts)");
    // println!("{} {}", paint_bold("select a variant:"), paint_white("(ts)"));
    paint_remind("select a variant:", "(javascript)");
    // println!("typescript(ts)");
    // println!("javascript(js)");
    println!("{}",paint_option("typescript"));
    println!("{}",paint_option("javascript"));
    let mut variant = read_line().to_lowercase();
    variant = match variant.as_str() {
        "ts" => { "typescript".to_string() }
        "typescript" => { "typescript".to_string() }
        "js" => { "javascript".to_string() }
        "javascript" => { "javascript".to_string() }
        _ => { "typescript".to_string() }
    };
    println!("{}", paint_user_input(&variant));

    let user_select = UserSelected::new(&project_name, &framework, &variant);
    // println!("{user_select:?}");

    user_select.init();

    let git = ask_git_init();
    install(&user_select.project_name, &ask_install());
    if git { git_init(project_name); }
}
