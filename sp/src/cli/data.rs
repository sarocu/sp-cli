pub mod data_ops {
    use std;
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::env;
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::{BufReader, Write};
    use std::path::Path;
    use std::path::PathBuf;

    use clap::{Arg, ArgMatches};
    use clap_nested;
    use colored::Colorize;

    extern crate csv;
    use csv::Reader;

    extern crate serde;
    extern crate serde_json;
    use serde::{Deserialize, Serialize};

    use dialoguer::{Checkboxes, Confirmation, Input};

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
            let mut data_path = matches.value_of("path").unwrap_or("");
            let copy_data = Confirmation::new().with_text("Copy file to project directory?").interact()?;

            println!("{}", "adding new dataset to project!".blue());
            let data_name = Input::<String>::new().with_prompt("What do you want to call this dataset?").interact()?;
            println!("{}", data_path);
            let mut csv_reader = Reader::from_path(data_path).unwrap();
            let headers = csv_reader.headers().unwrap();

            let mut save_path = data_path.to_owned();
            if copy_data {
                let mut new_path = env::current_dir().unwrap();
                new_path.push("data");
                new_path.push(format!("{}.csv", data_name));
                save_path = new_path.to_string_lossy().to_owned().to_string();
                fs::copy(data_path, new_path);
            }

            let mut select_data = Checkboxes::new();
            for column in headers {
                select_data.item(column);
            }

            let selected_items = select_data.item("All Columns").with_prompt(&format!("{}", "Which columns would you like to add to a dataframe? (hint: use space bar to select)".cyan())).interact()?;
            let config_path = find_sp_config();
            let test_stuff = vec!["things".to_string()];
            let mut fields_for_df = Vec::new();
            for index in selected_items {
                fields_for_df.push(headers[index].into());
            }
            add_to_sp(&save_path, &data_name, fields_for_df, config_path);
            Ok(())
        })
    }

    #[derive(Serialize, Deserialize)]
    struct Configs {
        dependencies: Vec<String>,
        entrypoint: String,
        project: String,
        python_interpreter: String,
        data: DataBlock,
    }

    #[derive(Serialize, Deserialize)]
    struct DataBlock {
        path: String,
        db: String,
        dataframe: RefCell<Vec<Dataframe>>,
    }

    impl DataBlock {
        pub fn add(&self, value: Dataframe) {
            self.dataframe.borrow_mut().push(value);
        }
    }

    #[derive(Serialize, Deserialize)]
    struct Dataframe {
        name: String,
        path: String,
        vars: Vec<String>,
    }

    // impl Dataframe {
    //     pub fn add(&self, value: String) {
    //         self.vars.borrow_mut().push(value);
    //     }
    // }

    pub fn add_to_sp(
        file_path: &str,
        dataset_name: &str,
        fieldnames: Vec<String>,
        config_path: PathBuf,
    ) {
        let mut sp_config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(config_path.clone())
            .unwrap();

        let reader = BufReader::new(sp_config_file);

        let sp_config: Configs = serde_json::from_reader(reader).unwrap();

        let new_df = Dataframe {
            name: dataset_name.to_string(),
            path: file_path.to_string(),
            vars: fieldnames,
        };
        sp_config.data.add(new_df);

        let mut updated_configs = OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_path)
            .unwrap();
        updated_configs.write(serde_json::to_string_pretty(&sp_config).unwrap().as_bytes());
    }

    pub fn find_sp_config() -> std::path::PathBuf {
        let mut cwd = env::current_dir().unwrap();
        cwd.push("sp.json");
        return cwd;
    }

    pub fn add_to_db() {}
}
