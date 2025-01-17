use std::path::Path;
use crate::common::{ask_git_init, ask_install, copy_dir_all, current_exe_pkg, git_init, install, paint_remind, paint_user_input, read_line};

#[derive(Debug)]
struct UserSelectedNuxtApp {
    project_name: String,
}

impl UserSelectedNuxtApp {
    fn new(
        project_name: &str,
    ) -> Self {
        let project_name = if project_name.is_empty() {
            "nuxt-app"
        } else { project_name };

        UserSelectedNuxtApp {
            project_name: project_name.to_string(),
        }
    }
    // 创建文件
    fn init(&self) {
        let path = "public/nuxt/".to_string();

        println!("复制: {}", current_exe_pkg() + &path);

        copy_dir_all(
            Path::new(
                &(current_exe_pkg() + &path)
            ),
            Path::new(&self.project_name),
        ).unwrap();
    }
}

pub fn create_nuxt_project() {
    // project name
    // println!("What is your project named? >> nuxt-app");
    paint_remind("What is your project named? >>", "nuxt-app");
    let project_name = read_line();
    if project_name.is_empty() {
        println!("{}", paint_user_input("nuxt-app"))
    } else { println!("{}", paint_user_input(&project_name)); }

    let user_select = UserSelectedNuxtApp::new(
        &project_name
    );

    user_select.init();

    // 因为npm install可能卡住, 用户如果按回车可能直接选择了默认选项
    let git = ask_git_init();

    install(&user_select.project_name, &ask_install());

    if git { git_init(&user_select.project_name); }
}
