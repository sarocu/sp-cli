#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_json;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::io::{Write, BufReader, BufRead, Error};

use clap::{Arg, App, SubCommand, ArgMatches};
use clap_nested::{Command, Commander, MultiCommand};

use dialoguer::Input;
use colored:: Colorize;

mod project;

static VERSION: &'static str = "0.1.0";
static AUTHOR: &'static str = "SAROCU";

fn main() {
    let project = Command::new("project")
        .description("Create a new project structure")
        .options(|app| {
            app.arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .global(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Set the project name"),
            )
        })
        .runner(|args: &str, matches: &ArgMatches<'_>| {
            let project_name = matches.value_of("name").unwrap_or("app");
            println!("Creating Project: {}", project_name.cyan());
            let mut path = PathBuf::from("./").join(project_name);
            fs::create_dir(path)?;
            println!("Project Path: {}", PathBuf::from("./").join(project_name).into_os_string().into_string().unwrap().blue());

            println!("{}", "Creating config and entrypoint files...".green());
            let mut config_file = fs::File::create(PathBuf::from("./").join(project_name).join("sp.json"))?;
            config_file.write_all(project::project_boilerplate::config_text(project_name).as_bytes())?;

            let mut entrypoint = fs::File::create(PathBuf::from("./").join(project_name).join("index.py"))?;
            entrypoint.write_all(project::project_boilerplate::entrypoint_text().as_bytes());
            let mut init_file = fs::File::create(PathBuf::from("./").join(project_name).join("__init__.py"))?;

            let mut setup_py = fs::File::create(PathBuf::from("./").join(project_name).join("setup.py"))?;
            
            println!("{}", "Creating directory structure...".green());
            let mut model_path = PathBuf::from("./").join(project_name).join("models");
            fs::create_dir(model_path)?;

            let mut test_path = PathBuf::from("./").join(project_name).join("tests");
            fs::create_dir(test_path)?;

            let mut output_path = PathBuf::from("./").join(project_name).join("output");
            fs::create_dir(output_path)?;

            let mut data_path = PathBuf::from("./").join(project_name).join("data");
            fs::create_dir(data_path)?;

            println!("🍻  {} 🍻", "Finished! Boilerplate project created".cyan());
            Ok(())
        });

    let multi_cmd: MultiCommand<(str), (str)> = Commander::new()
        .add_cmd(project)
        .into_cmd("new")
        .description("Generate new projects, models, tests, or data collections");

    Commander::new()
        .options(|app| {
            app.arg(
                Arg::with_name("environment")
                    .short("e")
                    .long("env")
                    .global(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Sets an environment value, defaults to \"dev\""),
            )
        })
        .args(|_args, matches| matches.value_of("environment").unwrap_or("dev"))
        .add_cmd(multi_cmd)
        .no_cmd(|_args, _matches| {
            println!("Nothing doing");
            Ok(())
        })
        .run()
        .unwrap();
}