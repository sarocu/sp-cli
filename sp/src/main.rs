#[macro_use]
extern crate clap;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::io::{Write, BufReader, BufRead, Error};

use clap::{Arg, App, SubCommand, ArgMatches};
use clap_nested::{Command, Commander, MultiCommand};

use dialoguer::Input;
use colored:: Colorize;

static VERSION: &'static str = "0.1.0";
static AUTHOR: &'static str = "SAROCU";

fn main() {
    // let sp_new = Command::new("new")
    //     .description("Scaffold out a new project, model, test, or data package")
    //     .options(|app| {
    //         app.arg(
    //             Arg::with_name("debug")
    //                 .short("d")
    //                 .help("Print out additional detail")
    //         )
    //     })
    //     .runner(|args: &str, matches: &ArgMatches<'_>| {
    //         let debug = clap::value_t!(matches, "debug", bool).unwrap_or_default();
    //         Ok(())
    //     });

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

            let mut config_file = fs::File::create(PathBuf::from("./").join(project_name).join("sp.json"))?;

            let mut model_path = PathBuf::from("./").join(project_name).join("models");
            fs::create_dir(model_path)?;
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


    // let matches = App::new("Superplus CLI")
    //     .version(VERSION)
    //     .author(AUTHOR)
    //     .about("Project Scaffolding for Scientific and Engineering Projects")
    //     .arg(Arg::with_name("debug")
    //         .short("d")
    //         .long("debug")
    //         .value_name("DEBUG_SETTING")
    //         .help("Toggles the debug setting - accepts true/false")
    //         .takes_value(true))
    //     .subcommand(SubCommand::with_name("new")
    //         .about("Generate a new project, model, test, or data model")
    //         .version(VERSION)
    //         .author(AUTHOR)
    //         .subcommand(SubCommand::with_name("project")
    //             .about("Create a new project directory and boilerplate structure")
    //             .version(VERSION)
    //             .author(AUTHOR)
    //             .arg(Arg::with_name("name")
    //                 .short("n")
    //                 .long("name")
    //                 .value_name("PROJECT_NAME")
    //                 .help("New Project Name")
    //                 .takes_value(true))
    //             )
    //         ).get_matches();

    // let scaffold_name = matches.value_of("name").unwrap_or("app");

    // if let Some(matches) = matches.subcommand_matches("new") {
    //     if matches.is_present("project") {
    //         let mut path = PathBuf::from("/").join(scaffold_name);
    //         fs::create_dir_all(path);
    //     }
    // }

    
    // if let Some(matches) = matches.subcommand_matches("project") {
    //     if matches.is_present("name") {
    //         let mut path = PathBuf::from("/").join(scaffold_name);
    //         fs::create_dir_all(path);
    //     }
    // }

}