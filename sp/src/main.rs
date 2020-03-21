#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_json;

use clap::{Arg, ArgMatches};
use clap_nested::{Command, Commander, MultiCommand};

use colored:: Colorize;

mod cli;

fn main() {
    let dataset = Command::new("dataset")
        .description("Add a new dataset to an existing superplus project")
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
            let data_name = matches.value_of("name").unwrap_or("datas");
            println!("{}", "adding new dataset to project!".blue());
            Ok(())
        });

    let multi_cmd: MultiCommand<(str), (str)> = Commander::new()
        .add_cmd(cli::project_boilerplate::project_cmd())
        .add_cmd(cli::package::package_boilerplate::package_cmd())
        .add_cmd(dataset)
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
            cli::version::version_info();
            Ok(())
        })
        .run()
        .unwrap();
}