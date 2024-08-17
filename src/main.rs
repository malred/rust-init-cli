use clap::{Parser, Subcommand};

// 读取输入
pub fn read_line() -> String {
    let mut input = String::new();
    // 遇到错误, 不关心, 只希望它遇到错误就停止(故障时执行panic)
    // std::io::stdin().read_line(&mut input).unwrap();
    // 与 unwrap 相比，expect 提供了相同的功能，但允许附加一个自定义的错误消息
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

use std::{io, fs};
use std::env::{current_dir, current_exe};
use std::path::{Path};

// 复制文件夹到指定路径
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    // fs::create_dir_all(&dst)?;
    fs::create_dir_all(&dst).expect("创建dst目录失败");

    // println!("遍历");
    fs::read_dir(&src).expect("找不到src目录");
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

// clap可以通过反射宏读取注释
#[derive(Subcommand)]
enum Commands {
    /// create.
    Create,
}

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
                panic!("No such framework type")
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
                panic!("No such framework type")
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
        let mut path = "public/".to_string();

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

        // 获取当前目录的路径
        let current_dir1 = env!("CARGO_MANIFEST_DIR");
        let current_exe = current_exe().unwrap();
        println!("current exe {:?}", current_exe);
        // let current_dir2 = env!("CARGO_HOME");
        // // let current_dir3 = env!("CARGO_BUILD_TARGET_DIR");
        // // let current_dir4 = env!("CARGO_TARGET_DIR");
        // println!("当前目录 env {:?}", current_dir1);
        // println!("当前目录 home {:?}", current_dir2);
        // println!("当前目录 path os {:?}", Path::new(".").parent().unwrap());
        // // println!("当前目录 target {:?}", current_dir3);
        // // println!("当前目录 target {:?}", current_dir4); not defined at compile time
        // println!("当前目录 {:?}", current_dir().unwrap().display());
        // println!("开始复制 {:?}", &path);
        // // println!("开始复制 {:?}", &(current_dir1.to_string() + "/" + &path));

        println!("复制 {}",
                 &(current_exe.display().to_string().replace("rust-vite-cli.exe", "")
                     + "/" + &path));

        // todo: 从网络上下载 或 调用cmd git clone

        copy_dir_all(
            // src,
            // Path::new(&(current_dir1.to_string() + "/" + &path)),
            Path::new(
                &(current_exe.display().to_string().replace("rust-vite-cli.exe", "")
                    + "/" + &path)
            ),
            Path::new(&self.project_name),
        ).unwrap();
    }
}

fn create_vite_project() {
    // project name
    println!("your project name?");
    let project_name = read_line();

    // select a framework
    // react vue ...
    println!("select a framework:");
    println!("react");
    println!("vue");
    let framework = read_line();
    let framework = framework.to_lowercase();

    // select a variant
    // javascript typescript ...
    println!("select a variant:");
    println!("typescript(ts)");
    println!("javascript(js)");
    let variant = read_line();
    let variant = variant.to_lowercase();

    let user_select = UserSelected::new(&project_name, &framework, &variant);
    println!("{user_select:?}");

    user_select.init()
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::Create) => {
            // println!("List users here")
            create_vite_project()
        }
        None => {
            println!("Run with --help to see instructions.")
        }
    }
}