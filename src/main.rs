use std::{env, fs};
use std::path::PathBuf;
use clap::Parser;
use askama::Template;
use crate::config::ModuleLanguage;

mod config;

fn main() {
    let cfg = config::Config::parse();

    let output_directory = match cfg.output_directory {
        Some(dir) => PathBuf::from(dir),
        None =>  env::current_dir().unwrap(),
    };

    let idea_dir = output_directory.join(".idea");

    // check idea_dir exists
    if !idea_dir.exists() {
        fs::create_dir_all(&idea_dir).unwrap();
    }

    let module_name = match cfg.module_name {
        Some(name) => name,
        None => env::current_dir().unwrap().file_name().unwrap().to_str().unwrap().to_string(),
    };

    let path = idea_dir.join(format!("{}.iml", module_name));
    let contents;
    match cfg.module_language {
        ModuleLanguage::Rust => {
            contents = RustModule {}.render().unwrap();
        },
        ModuleLanguage::Go => {
            contents = GoModule {}.render().unwrap();
        },
        ModuleLanguage::Web => {
            contents = WebModule {}.render().unwrap();
        }
    }
    fs::write(path, contents).unwrap();

    let modules_file = idea_dir.join("modules.xml");
    let modules = Modules { module_name: &module_name }.render().unwrap();
    fs::write(modules_file, modules).unwrap();

    println!("created modules.xml and {}.iml in {}", module_name, idea_dir.display());
}

#[derive(Template)]
#[template(path = "rust-module.iml", escape = "none")]
struct RustModule {}

#[derive(Template)]
#[template(path = "go-module.iml", escape = "none")]
struct GoModule {}

#[derive(Template)]
#[template(path = "web-module.iml", escape = "none")]
struct WebModule {}

#[derive(Template)]
#[template(path = "modules.xml", escape = "none")]
struct Modules<'a> {
    module_name: &'a str,
}

pub fn render_template() -> String {
    let template = Modules { module_name: "World" };
    let output = template.render().unwrap();
    output
}
