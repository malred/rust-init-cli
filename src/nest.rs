use std::path::Path;
use crate::common::{copy_dir_all, current_exe_pkg, git_init, install, read_line};

#[derive(Debug)]
struct UserSelectedNestApp {
    project_name: String,
}

impl UserSelectedNestApp {
    fn new(
        project_name: &str,
    ) -> Self {
        let project_name = if project_name.is_empty() {
            "project-name"
        } else { project_name };

        UserSelectedNestApp {
            project_name: project_name.to_string(),
        }
    }
    // 创建文件
    fn init(&self) {
        let path = "public/nest/".to_string();

        println!("复制: {}", current_exe_pkg() + &path);

        copy_dir_all(
            Path::new(
                &(current_exe_pkg() + &path)
            ),
            Path::new(&self.project_name),
        ).unwrap();
    }
}

pub fn create_nest_project() {
    // project name
    println!("What is your project named? >> project-name");
    let project_name = read_line();

    let user_select = UserSelectedNestApp::new(
        &project_name
    );
    // println!("{user_select:?}");

    user_select.init();

    install(&user_select.project_name);

    git_init(&user_select.project_name);
}
