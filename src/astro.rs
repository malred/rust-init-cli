use std::path::Path;
use ansi_term::Color::{Green, Purple, White};
use ansi_term::{Color, Style};
use crate::common::{ask_git_init, ask_install, copy_dir_all, current_exe_pkg, git_init, install, match_bool, paint_bold, paint_option, paint_remind, paint_remind_with_other, paint_success, paint_underline_white, paint_user_input, paint_warning, read_line};

#[derive(Debug)]
struct UserSelectedAstroApp {
    project_name: String,
    tmpl: Template,
    ts: bool,
    ts_mode: Option<Mode>,
}

impl UserSelectedAstroApp {
    fn new(
        project_name: &str,
        tmpl: Template,
        ts: bool,
        mode: Option<Mode>,
    ) -> Self {
        let project_name = if project_name.is_empty() {
            "project-name" // todo: random str
        } else { project_name };

        UserSelectedAstroApp {
            project_name: project_name.to_string(),
            tmpl,
            ts,
            ts_mode: mode,
        }
    }
    // 创建文件
    fn init(&self) {
        let mut path = "public/astro/astro-".to_string();

        match self.tmpl {
            Template::Sample => path += "sample",
            Template::Blog => path += "blog",
            Template::Empty => path += "empty"
        }

        path += "-";

        if self.ts {
            path += "ts-";
            match &self.ts_mode {
                None => { path += "relaxed" }
                Some(mode) => {
                    match mode {
                        Mode::Strict => { path += "strict" }
                        Mode::Strictest => { path += "strictest" }
                        Mode::Relaxed => { path += "relaxed" }
                    }
                }
            }
        } else {
            path += "js"
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

#[derive(Debug)]
enum Template {
    Sample,
    Blog,
    Empty,
}

#[derive(Debug)]
enum Mode {
    Strict,
    Strictest,
    Relaxed,
}

pub fn create_astro_project() {
    // project name (astro 使用的是随机名称生成)
    // println!("dir: Where should we create your new project? (project-name)");
    print!(" {} ", Style::new().on(Color::Purple).paint(" dir "));
    paint_remind("Where should we create your new project? >>", "project-name");
    let project_name = read_line();
    if project_name.is_empty() {
        println!("{}", paint_user_input("project-name"))
    } else { println!("{}", paint_user_input(&project_name)); }

    // template
    // print!("{}", Style::new().on(Color::Purple).fg(White).paint(" dir "));
    print!(" {} ", Style::new().on(Color::Purple).paint(" tmpl "));
    // println!("How would you like to start your new project? (input 1,2,3 to select)");
    println!("{}", paint_bold("How would you like to start your new project?"));
    println!("\t1. {}", paint_underline_white("Include sample files (recommended)"));
    println!("\t2. {}", paint_option("Use blog template"));
    println!("\t3. {}", paint_option("Empty"));
    let tmpl = read_line().to_lowercase();
    let tmpl = match tmpl.to_lowercase().as_str() {
        "1" => {
            println!("{}", paint_user_input("Include sample files (recommended)"));
            Template::Sample
        }
        "2" => {
            println!("{}", paint_user_input("Use blog template"));
            Template::Blog
        }
        "3" => {
            println!("{}", paint_user_input("Empty"));
            Template::Empty
        }
        _ => {
            println!("{}", paint_user_input("Include sample files (recommended)"));
            Template::Sample
        }
    };

    // ts
    // println!("ts: Do you plan to write TypeScript? yes(default)/no");
    print!(" {} ", Style::new().on(Color::Purple).paint(" ts "));
    paint_remind_with_other(
        "Do you plan to write TypeScript?", "", "yes", "/no",
    );
    let ts = read_line().to_lowercase();
    let ts = match_bool(ts.as_str().clone());
    if !ts {
        println!("{}", paint_warning(
            "◼ No worries! TypeScript is supported in Astro by default, but you are free to continue writing JavaScript instead.")
        );
    }

    // use
    let mut to_use: Mode = Mode::Strict;
    if ts {
        // println!("use: How strict should TypeScript be? input 1,2,3 to select");
        paint_remind("use: How strict should TypeScript be?", "");
        // println!("\t1. Strict (recommended & default)");
        println!("\t1. {}", paint_underline_white("Strict (recommended)"));
        println!("\t2. {}", paint_option("Strictest"));
        println!("\t3. {}", paint_option("Relaxed"));
        let mode = read_line().to_lowercase();
        to_use = match mode.to_lowercase().as_str() {
            "1" => {
                println!("{}", paint_user_input("Strict (recommended)"));
                Mode::Strict
            }
            "2" => {
                println!("{}", paint_user_input("Strictest"));
                Mode::Strictest
            }
            "3" => {
                println!("{}", paint_user_input("Relaxed"));
                Mode::Relaxed
            }
            _ => {
                println!("{}", paint_user_input("Strict (recommended)"));
                Mode::Strict
            }
        }
    }

    // deps
    // println!("deps: Install dependencies? yes(default)/no");
    // let deps = read_line();
    // let deps = match deps.to_lowercase().as_str() {
    //     "yes" => true,
    //     "no" => false,
    //     _ => true
    // };
    let npm_type = ask_install();

    // git
    let git = ask_git_init();

    let user_select = UserSelectedAstroApp::new(
        &project_name, tmpl, ts, if ts { Some(to_use) } else { None },
    );
    // println!("{user_select:?}");

    user_select.init();

    install(&user_select.project_name, &npm_type);

    if git { git_init(&user_select.project_name); }
}

