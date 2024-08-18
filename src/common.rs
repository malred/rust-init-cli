use std::{fs, io, process};
use std::env::current_exe;
use std::path::Path;
use std::process::Command;


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

// 执行npm install操作
pub fn install(project_name: &str) {
    println!("which package manager would you like to use >> npm(default)/pnpm/yarn");
    let mut npm_type = read_line();
    if npm_type.is_empty() {
        npm_type = "npm".to_string()
    }

    println!("start install ...");

    if cfg!(target_os = "windows") {
        // 获取当前目录
        let mut out = Command::new("cmd").arg("/c")
            .arg(
                "chdir"
            )
            .output().expect("cmd exec error!");

        let strs = String::from_utf8_lossy(&out.stdout);
        let chdir = strs.replace("\r\n", "");
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
                 "cd ".to_string() + &chdir + "\\" + project_name + " && " +
                     // npm_type + " install --prefix " + &current_exe_pkg() + project_name
                     &npm_type + " install"
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

// 执行git init操作
pub fn git_init(project_name: &str) {
    println!("Initialize git repository? No(default)/Yes");
    let mut use_git_init = read_line();
    let mut use_git_init = use_git_init.to_lowercase();
    let use_git_init = match use_git_init.as_str() {
        "yes" => {
            true
        }
        "no" => {
            false
        }
        _ => {
            true
        }
    };

    if !use_git_init {
        return;
    }

    println!("start git init ...");

    if cfg!(target_os = "windows") {
        let mut out = Command::new("cmd").arg("/c")
            .arg(
                "chdir"
            )
            .output().expect("cmd exec error!");

        let strs = String::from_utf8_lossy(&out.stdout);
        let chdir = strs.replace("\r\n", "");

        let mut out = Command::new("cmd").arg("/c")
            .arg(
                "cd ".to_string() + chdir.as_str() + "\\" + project_name +
                    " && " + "git init"
            )
            .spawn().expect("cmd exec error!");
        // .output().expect("cmd exec error!");

        println!("{}",
                 "cd ".to_string() + &chdir + "\\" + project_name +
                     " && " + "git init"
        );

        let status = out.wait().expect("failed to wait for child");

        if status.success() {
            println!("done.");
        }
    } else {
        println!("暂不支持mac os");
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