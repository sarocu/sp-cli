#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_json;

use clap::{Arg};
use clap_nested::{Commander, MultiCommand};

mod cli;

fn main() {
    let multi_cmd: MultiCommand<(str), (str)> = Commander::new()
        .add_cmd(cli::project_boilerplate::project_cmd())
        .add_cmd(cli::package::package_boilerplate::package_cmd())
        .add_cmd(cli::charts::viz::chart_cmd())
        .add_cmd(cli::data::data_ops::csv_cmd())
        .add_cmd(cli::models::model_ops::model_cmd())
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
