pub mod model_ops {
    use std;
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::{BufReader, Write};
    use std::path::Path;
    use std::path::PathBuf;

    use clap::{Arg, ArgMatches};
    use clap_nested;
    use colored::Colorize;

    extern crate serde;
    extern crate serde_json;
    use serde::{Deserialize, Serialize};

    use dialoguer::{Checkboxes, Confirmation, Input};

    use crate::cli;

    pub fn model_cmd<'a>() -> clap_nested::Command<'a, str> {
        clap_nested::Command::new("model")
            .description("Add new model boilerplate to an existing superplus project")
            .options(|app| {
                app.arg(
                    Arg::with_name("modeltype")
                        .short("t")
                        .long("type")
                        .global(true)
                        .takes_value(true)
                        .value_name("STRING")
                        .help("type of learning model to create"),
                )
            })
            .runner(|args: &str, matches: &ArgMatches<'_>| {
                let model_type = matches.value_of("modeltype").unwrap_or("");

                println!("Creating a new {} model", model_type.cyan());

                let dispatch = {
                    let mut creator: HashMap<String, String> = HashMap::new();
                    creator.insert("simple-linear".into(), simple_linear());
                    creator.insert("vector-ar".into(), simple_linear());
                    creator
                };

                let new_model_text = &dispatch[&model_type.to_string()];
                let config_path = cli::data::data_ops::find_sp_config();
                println!("updating configs at: {}", config_path.to_string_lossy());
                let mut new_model_path = env::current_dir().unwrap();
                new_model_path.push("models");
                new_model_path.push(format!("{}.py", model_type));

                let save_path = new_model_path.clone();

                let mut model_file = fs::File::create(new_model_path)?;
                model_file.write_all(new_model_text.as_bytes());

                add_to_sp(&save_path.to_string_lossy(), &model_type, config_path);

                Ok(())
            })
    }

    pub fn add_to_sp(file_path: &str, model_name: &str, config_path: PathBuf) {
        let mut sp_config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(config_path.clone())
            .unwrap();

        let reader = BufReader::new(sp_config_file);

        let sp_config: cli::data::data_ops::Configs = serde_json::from_reader(reader).unwrap();
        let new_model = cli::data::data_ops::ModelClass {
            name: model_name.to_string(),
            path: file_path.to_string(),
        };
        sp_config.models.add(new_model);

        let mut updated_configs = OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_path)
            .unwrap();
        updated_configs.write(serde_json::to_string_pretty(&sp_config).unwrap().as_bytes());
    }

    pub fn simple_linear() -> std::string::String {
        return String::from(
            "import pandas
from sklearn.linear_model import LinearRegression


class SimpleLinear:
    def __init__(self, predictorKey, responseKey):
        self.predictor = predictorKey
        self.response = responseKey

    def setData(self, predictorArray, responseArray):
        self.x = predictorArray.values.reshape(-1, 1)
        self.y = responseArray.values.reshape(-1, 1)

    def results(self):
        self.fit = LinearRegression().fit(self.x, self.y)

    def runTests(self):
        test_set = []

        # Standard score method, includes R2:
        test_set.append({\"test\": \"r2\", \"results\": self.fit.score(self.x, self.y)})

        return test_set",
        );
    }

    pub fn vector_ar() -> std::string::String {
        return String::from(
            "import statsmodels.api as sm
import numpy


class VectorAutoRegressive:
    def __init__(self, maxLags, criterion):
        self.lags = maxLags
        self.ic = criterion

    def setData(self, timeseries):
        self.model = sm.tsa.VAR(timeseries)

    def results(self):
        self.fit = self.model.fit(maxlags=self.lags, ic=self.ic)

    def runTests(self):
        test_set = []

        test_set.append({\"test\": \"lag-order\", \"value\": self.fit.k_ar})

        return test_set",
        );
    }

}
