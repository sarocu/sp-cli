pub mod data_ops {
    use std;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    use std::env;
    use std::io::{Write, BufReader, BufRead};

    use clap_nested;
    use clap::{Arg, App, SubCommand, ArgMatches};

    use colored:: Colorize;

    extern crate csv;
    use csv::Reader;

    extern crate serde_json;
    extern crate serde;
    use serde::{Deserialize, Serialize};
    use serde::ser::Serializer;

    use dialoguer::{Input, Confirmation, Checkboxes, Select};

    pub fn csv_cmd<'a>() -> clap_nested::Command<'a, str> {
        clap_nested::Command::new("csv")
        .description("Add a new dataset to an existing superplus project")
        .options(|app| {
            app.arg(
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .global(true)
                    .takes_value(true)
                    .value_name("STRING")
                    .help("Path to the file"),
            )
        })
        .runner(|args: &str, matches: &ArgMatches<'_>| {
            let data_path = matches.value_of("path").unwrap_or("");
            let copy_data = Confirmation::new().with_text("Copy file to project directory?").interact()?;
            println!("{}", "adding new dataset to project!".blue());
            println!("{}", data_path);
            let mut csv_reader = Reader::from_path(data_path).unwrap();
            let headers = csv_reader.headers().unwrap();
            
            let mut select_data = Checkboxes::new();
            for column in headers {
                select_data.item(column);
            }

            select_data.item("All Columns").with_prompt(&format!("{}", "Which columns would you like to add to a dataframe? (hint: use space bar to select)".cyan())).interact()?;

            Ok(())
        })
    }

    #[derive(Serialize, Deserialize)]
    struct Configs {
        dependencies: Vec<String>,
        entrypoint: String,
        project: String, 
        python_interpreter: String, 
        data: DataBlock
    }

    #[derive(Serialize, Deserialize)]
    struct DataBlock {
        path: String,
        db: String,
        dataframe:Dataframe
    }

    #[derive(Serialize, Deserialize)]
    struct Dataframe {
        name: String,
        path: String,
        vars: Vec<String>,
    }

    



    pub fn add_to_sp(file_path: &str, dataset_name: &str, fieldnames: Vec<String>, config_path: &str) {
        let sp_config_file = fs::File::open(file_path)
            .expect("file should be json");
        let sp_config: Configs = serde_json::from_reader(sp_config_file)
            .expect("Contents should be valid json");
        let data = sp_config.data;
        println!("{}", data.path);
    }

    pub fn find_sp_config() -> std::string::String {
        return String::from("sp.json");
    }

    pub fn add_to_db() {

    }
}