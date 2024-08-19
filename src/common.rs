use std::{fs, io, process};
use std::env::current_exe;
use std::io::Read;
use std::path::Path;
use std::process::Command;
use ansi_term::{ANSIGenericString, Color, Style};

// 复制文件夹到指定路径
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
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

    // println!("copy dir done.");
    Ok(())
}

// 读取输入
pub fn read_line() -> String {
    let mut input = String::new();
    // 遇到错误, 不关心, 只希望它遇到错误就停止(故障时执行panic)
    // std::io::stdin().read_line(&mut input).unwrap();
    // 与 unwrap 相比，expect 提供了相同的功能，但允许附加一个自定义的错误消息
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

// 询问是否install
pub fn ask_install() -> String {
    // println!("Install dependencies? yes(default)/no");
    println!("{} {}/no", paint_bold("Install dependencies?"), paint_white("yes"));
    let deps = read_line();
    let deps = match deps.to_lowercase().as_str() {
        "yes" => true,
        "y" => true,
        "1" => true,
        "no" => false,
        "n" => false,
        "2" => false,
        _ => true
    };

    if deps {
        println!("{}", paint_user_input("yes"));
        // println!("which package manager would you like to use >> 1.pnpm(default)/2.npm/3.yarn",
        println!("{} >> {}/2.npm/3.yarn",
                 paint_bold("which package manager would you like to use"),
                 paint_white("1.pnpm"));
        let mut npm_type = read_line();
        let npm_type = match npm_type.to_lowercase().as_str() {
            "1" => { "npm" }
            "npm" => { "npm" }
            "n" => { "npm" }
            "2" => { "pnpm" }
            "pnpm" => { "pnpm" }
            "p" => { "pnpm" }
            "3" => { "yarn" }
            "yarn" => { "yarn" }
            "y" => { "yarn" }
            _ => { "pnpm" }
        };
        println!("{}", paint_user_input(npm_type));
        npm_type.to_string()
    } else {
        println!("{}", paint_user_input("no"));
        "".to_string()
    }
}

// 执行npm install操作
pub fn install(project_name: &str, npm_type: &str) {
    if npm_type.is_empty() {
        return;
    }
    println!("{}", paint_info("start install ..."));

    if cfg!(target_os = "windows") {
        // 获取当前目录
        let chdir = cmd("chdir").unwrap();
        let chdir = chdir.replace("\r\n", "");

        // let mut out = Command::new("cmd").arg("/c")
        //     .arg(
        //         "chdir"
        //     )
        //     .output().expect("cmd exec error!");
        //
        // let strs = String::from_utf8_lossy(&out.stdout);
        // let chdir = strs.replace("\r\n", "");
        // println!("chdir {:?}", chdir.clone() + "\\" + project_name);
        // println!("chdir os {:?}", " G:\\code-g\\rust-vite-cli\\my-app");

        // 进入当前目录 && install
        let mut out = Command::new("cmd").arg("/c")
            .arg(
                // "cd ".to_string() + &chdir + project_name + " && " +
                // "cd  G:\\code-g\\rust-vite-cli\\my-app".to_string() + " && " + // ok
                "cd ".to_string() + chdir.as_str() + "\\" + project_name + " && " +
                    // npm_type + " install --prefix " + &current_exe_pkg() + project_name
                    &npm_type + " install"
            )
            .spawn().expect("cmd exec error!");
        // .output().expect("cmd exec error!");

        println!("{}",
                 paint_info(&(
                     "cd ".to_string() + &chdir + "\\" + project_name + " && " +
                         // npm_type + " install --prefix " + &current_exe_pkg() + project_name
                         &npm_type + " install"
                 ))
        );

        let status = out.wait().expect("failed to wait for child");

        // println!("child exited with: {}", status);
        if status.success() {
            println!("done.");
        }
    } else {
        println!("暂不支持mac os");
    }
}

pub fn ask_git_init() -> bool {
    // println!("Initialize git repository? No(default)/Yes");
    println!("{} {}/yes", paint_bold("Initialize git repository?"), paint_white("no"));
    let mut use_git_init = read_line();
    let mut use_git_init = use_git_init.to_lowercase();
    let use_git_init = match use_git_init.as_str() {
        "yes" => {
            true
        }
        "y" => {
            true
        }
        "1" => {
            true
        }
        "no" => {
            false
        }
        "n" => {
            false
        }
        "2" => {
            false
        }
        _ => {
            false
        }
    };
    if use_git_init {
        println!("{}", paint_user_input("yes"));
    } else { println!("{}", paint_user_input("no")); }
    use_git_init
}

// 执行命令行操作
pub fn cmd(cmd_shell: &str) -> Option<String> {
    if cfg!(target_os = "windows") {
        let mut out = Command::new("cmd").arg("/c")
            .arg(cmd_shell)
            .output().expect("cmd exec error!");

        // println!("{}", cmd_shell);
        Some(String::from_utf8_lossy(&out.stdout).to_string())

        // let status = out.wait().expect("cmd exec error!");
        //
        // if status.success() {
        //     println!("done.");
        // }
    } else {
        println!("{}", Color::Yellow.paint("暂不支持mac os"));
        None
    }
}

// 执行git init操作
pub fn git_init(project_name: &str) {
    println!("{}", paint_info("start git init ..."));

    if cfg!(target_os = "windows") {
        let chdir = cmd("chdir").unwrap();
        let chdir = chdir.replace("\r\n", "");

        // let mut out = Command::new("cmd").arg("/c")
        //     .arg(
        //         "chdir"
        //     )
        //     .output().expect("cmd exec error!");
        //
        // let strs = String::from_utf8_lossy(&out.stdout);
        // let chdir = strs.replace("\r\n", "");

        let mut out = Command::new("cmd").arg("/c")
            .arg(
                "cd ".to_string() + chdir.as_str() + "\\" + project_name +
                    " && " + "git init"
            )
            .spawn().expect("cmd exec error!");
        // .output().expect("cmd exec error!");

        println!("{}",
                 paint_info(&("cd ".to_string() + &chdir + "\\" + project_name +
                     " && " + "git init"))
        );

        let status = out.wait().expect("failed to wait for child");

        if status.success() {
            println!("{}", paint_success("done."));
        }
    } else {
        println!("{}", Color::Yellow.paint("暂不支持mac os"));
    }
}

pub fn current_exe_pkg() -> String {
    /*
    // 获取当前目录的路径
    // let current_dir1 = env!("CARGO_MANIFEST_DIR");
    // let current_exe = current_exe().unwrap();
    // println!("current exe {:?}", current_exe);
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
    // let pkg_name = env!("CARGO_PKG_NAME");
    // println!("{pkg_name}.exe");
    // let pkg_name = pkg_name.to_string() + ".exe";
    */

    let pkg_name = env!("CARGO_PKG_NAME");
    // println!("{pkg_name}.exe");
    let pkg_name = pkg_name.to_string() + ".exe";

    // 获取当前目录的路径
    let current_exe = current_exe().unwrap();
    current_exe.display().to_string().replace(&pkg_name, "")
}

// ansi_term 美化控制台print

pub fn paint_bold(str: &str) -> String {
    Style::new().bold().paint(str).to_string()
}

pub fn paint_white(str: &str) -> String {
    // 这个white颜色实际上类似gray灰色
    Color::White.paint(str).to_string()
}

pub fn paint_info(str: &str) -> String {
    Color::Blue.paint(str).to_string()
    // Color::Cyan.paint(str).to_string()
}

pub fn paint_user_input(str: &str) -> String {
    // Color::Green.paint(str.to_owned() + "\n").to_string()
    Color::Green.paint(str).to_string()
}

pub fn paint_success(str: &str) -> String {
    Color::Green.paint(str).to_string()
}

pub fn paint_option(str: &str) -> String {
    Color::Purple.paint(str).to_string()
}

// 打印提示
pub fn paint_remind_with_other(title: &str, default: &str, other: &str) {
    println!("{} {}{other}", paint_bold(title), paint_white(default));
}
pub fn paint_remind(title: &str, default: &str) {
    println!("{} {}", paint_bold(title), paint_white(default));
}