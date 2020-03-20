use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::io::{Write, BufReader, BufRead, Error};

extern crate clap;
use clap::{Arg, App, SubCommand};

static VERSION: &'static str = "0.1.0";
static AUTHOR: &'static str = "SAROCU";

fn main() {
    let matches = App::new("Superplus CLI")
        .version(VERSION)
        .author(AUTHOR)
        .about("Project Scaffolding for Scientific and Engineering Projects")
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .value_name("DEBUG_SETTING")
            .help("Toggles the debug setting - accepts true/false")
            .takes_value(true))
        .subcommand(SubCommand::with_name("new")
            .about("Generate a new project, model, test, or data model")
            .version(VERSION)
            .author(AUTHOR)
            .subcommand(SubCommand::with_name("project")
                .about("Create a new project directory and boilerplate structure")
                .version(VERSION)
                .author(AUTHOR)
                .arg(Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .value_name("PROJECT_NAME")
                    .help("New Project Name")
                    .takes_value(true))
                )
            ).get_matches();

    let scaffold_name = matches.value_of("name").unwrap_or("app");
    
    if let Some(matches) = matches.subcommand_matches("project") {
        if matches.is_present("name") {
            let mut path = PathBuf::from("/").join(scaffold_name);
            fs::create_dir_all(path);
        }
    }

}