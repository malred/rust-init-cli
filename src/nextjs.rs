use std::path::Path;
use crate::common::{ask_install, copy_dir_all, current_exe_pkg, install, read_line};

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
    println!("What is your project named? >> my-app");
    let project_name = read_line();

    println!("Would you like to use TypeScript? >> No/Yes(default)");
    let is_typescript = read_line();
    let is_typescript = is_typescript.to_lowercase();
    let is_typescript = match is_typescript.as_str() {
        "yes" => { true }
        "no" => { false }
        _ => { true }
    };

    println!("Would you like to use ESLint? >> No/Yes(default)");
    let use_eslint = read_line();
    let use_eslint = use_eslint.to_lowercase();
    let use_eslint = match use_eslint.as_str() {
        "yes" => { true }
        "no" => { false }
        _ => { true }
    };

    println!("Would you like to use Tailwind CSS? >> No/Yes(default)");
    let tailwind = read_line();
    let tailwind = tailwind.to_lowercase();
    let tailwind = match tailwind.as_str() {
        "yes" => { true }
        "no" => { false }
        _ => { true }
    };

    println!("Would you like to use `src/` directory? >> No(default)/Yes");
    let src = read_line();
    let src = src.to_lowercase();
    let src = match src.as_str() {
        "yes" => { true }
        "no" => { false }
        _ => { false }
    };

    println!("Would you like to use App Router? (recommended) >> No/Yes(default)");
    let app_r = read_line();
    let app_r = app_r.to_lowercase();
    let app_r = match app_r.as_str() {
        "yes" => { true }
        "no" => { false }
        _ => { true }
    };

    println!("Would you like to customize the default import alias (@/*)? >> No(default)/Yes");
    let use_alias = read_line();
    let use_alias = use_alias.to_lowercase();
    let use_alias = match use_alias.as_str() {
        "yes" => { true }
        "no" => { false }
        _ => { false }
    };
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

    let user_select = UserSelectedNextApp::new(
        &project_name, is_typescript, use_eslint, tailwind, src, app_r, use_alias,
    );
    // println!("{user_select:?}");

    user_select.init();

    // spawn(move ||
    // install(&user_select.project_name, &npm_type)
    // ).join().unwrap();
    install(&user_select.project_name, &ask_install());
}
