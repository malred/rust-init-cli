use std::path::Path;
use crate::common::{ask_git_init, ask_install, copy_dir_all, current_exe_pkg, git_init, install, match_bool, match_bool_default_no, paint_option, paint_remind, paint_remind_with_other, paint_user_input, read_line};

#[derive(Debug)]
struct UserSelectedNextApp {
    project_name: String,
    is_typescript: bool,
    use_eslint: bool,
    use_tailwindcss: bool,
    use_src_dir: bool,
    use_app_router: bool,
    import_alias: bool,
    // import_alias: String,
}

impl UserSelectedNextApp {
    fn new(
        project_name: &str,
        is_ts: bool,
        eslint: bool,
        tailwind: bool,
        src: bool,
        app_r: bool,
        // alias: &str,
        alias: bool,
    ) -> Self {
        let project_name = if project_name.is_empty() {
            "my-app"
        } else { project_name };

        UserSelectedNextApp {
            project_name: project_name.to_string(),
            is_typescript: is_ts,
            use_eslint: eslint,
            use_tailwindcss: tailwind,
            use_src_dir: src,
            use_app_router: app_r,
            // import_alias: alias.to_string(),
            import_alias: alias,
        }
    }
    // 创建文件
    fn init(&self) {
        // let mut path = "./src/public/".to_string();
        let mut path = "public/nextjs/".to_string();

        if self.is_typescript {
            path += "ts";
        } else {
            path += "js";
        }

        if self.use_eslint {
            path += "-";
            path += "lint";
        }
        if self.use_tailwindcss {
            path += "-";
            path += "tailwind";
        }
        if self.use_src_dir {
            path += "-";
            path += "src";
        }
        if self.use_app_router {
            path += "-";
            path += "app";
        }
        if self.import_alias {
            path += "-";
            path += "alias";
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

pub fn create_next_project() {
    // project name
    // println!("What is your project named? >> my-app");
    paint_remind("What is your project named? >> ", "my-app");
    let mut project_name = read_line();
    project_name = if project_name.is_empty() { "my-app".to_string() } else { project_name };
    println!("{}", paint_user_input(&project_name));

    // println!("Would you like to use TypeScript? >> No/Yes(default)");
    paint_remind_with_other(
        "Would you like to use TypeScript? >> ", "", "Yes", "/No",
    );
    let is_typescript = read_line().to_lowercase();
    let is_typescript = match_bool(is_typescript.as_str().clone());

    // println!("Would you like to use ESLint? >> No/Yes(default)");
    paint_remind_with_other(
        "Would you like to use ESLint? >> ", "", "Yes", "/No",
    );
    let use_eslint = read_line().to_lowercase();
    let use_eslint = match_bool(use_eslint.as_str().clone());

    // println!("Would you like to use Tailwind CSS? >> No/Yes(default)");
    paint_remind_with_other(
        "Would you like to use Tailwind CSS? >> ", "", "Yes", "/No",
    );
    let tailwind = read_line().to_lowercase();
    let tailwind = match_bool(tailwind.as_str().clone());

    // println!("Would you like to use `src/` directory? >> No(default)/Yes");
    paint_remind_with_other(
        "Would you like to use `src/` directory? >> ", "", "No", "/Yes",
    );
    let src = read_line().to_lowercase();
    let src = match_bool_default_no(src.as_str().clone());

    // println!("Would you like to use App Router? (recommended) >> No/Yes(default)");
    paint_remind_with_other(
        "Would you like to use App Router? (recommended) >> ", "", "Yes", "/No",
    );
    let app_r = read_line().to_lowercase();
    let app_r = match_bool(app_r.as_str().clone());

    // println!("Would you like to customize the default import alias (@/*)? >> No(default)/Yes");
    paint_remind_with_other(
        "Would you like to customize the default import alias (@/*)? >> ",
        "", "No", "/Yes",
    );
    let use_alias = read_line().to_lowercase();
    let use_alias = match_bool_default_no(use_alias.as_str().clone());
    {
        // let alias_name = match use_alias.as_str() {
        //     "yes" => {
        //         println!("What import alias would you like configured? » @/* (default)");
        //         let alias = read_line();
        //         alias
        //     }
        //     "no" => {
        //         "".to_string()
        //     }
        //     _ => {
        //         println!("What import alias would you like configured? » @/* (default)");
        //         let alias = read_line();
        //         alias
        //     }
        // };
        // println!("{}", alias_name.is_empty());
    }

    let user_select = UserSelectedNextApp::new(
        &project_name, is_typescript, use_eslint, tailwind, src, app_r, use_alias,
    );
    // println!("{user_select:?}");

    user_select.init();

    let git = ask_git_init();
    install(&user_select.project_name, &ask_install());
    if git { git_init(&user_select.project_name) }
}
