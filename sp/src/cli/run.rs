pub mod run_ops {
    use clap::{Arg, ArgMatches};
    use clap_nested;
    use colored::Colorize;

    use std;
    use std::fs::metadata;
    use std::fs::OpenOptions;
    use std::io::{BufReader, Write};
    use std::path::PathBuf;

    extern crate csv;
    use csv::Reader;

    extern crate serde;
    extern crate serde_json;
    use serde::{Deserialize, Serialize};

    use crate::cli;

    pub fn run_cmd<'a>() -> clap_nested::Command<'a, str> {
        clap_nested::Command::new("run")
            .description("Run script commands described in sp.json")
            .options(|app| app.arg(Arg::with_name("scripts").multiple(true)))
            .runner(|args: &str, matches: &ArgMatches<'_>| {
                let scripts = matches.values_of("scripts").unwrap();
                println!("{:?}", scripts);

                for script in scripts {
                    let config_path = cli::data::data_ops::find_sp_config();
                    let script_cmd = get_script(script, config_path);
                    println!("{}", script_cmd);
                    let mut run_script = std::process::Command::new("bash");
                    if let Ok(mut cmd) = run_script.arg("-c").arg(script_cmd).spawn() {
                        cmd.wait().expect("command execution failed");
                        println!("{}", "sp run script complete".cyan());
                    } else {
                        println!("{}", "script could not be run".red());
                    }
                }
                Ok(())
            })
    }

    pub fn get_script(script_name: &str, config_path: PathBuf) -> std::string::String {
        let is_sp = match metadata(&config_path) {
            Ok(is_sp) => println!("{}", "getting command...".cyan()),
            Err(e) => panic!("{}", "Could not locate sp.json".red()),
        };
        let sp_config_file = OpenOptions::new()
            .read(true)
            .open(config_path.clone())
            .unwrap();

        let reader = BufReader::new(sp_config_file);

        let sp_config: cli::data::data_ops::Configs = serde_json::from_reader(reader).unwrap();
        match sp_config.scripts.get(script_name) {
            Some(script) => return script.to_string(),
            None => panic!("Could not find script {}", script_name.red()),
        }
    }
}
