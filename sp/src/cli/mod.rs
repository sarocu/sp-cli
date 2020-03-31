pub mod charts;
pub mod data;
pub mod package;
pub mod version;
pub mod models;

pub mod project_boilerplate {
    use std;
    use std::env;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    use clap::{Arg, ArgMatches};
    use clap_nested;

    use colored::Colorize;

    extern crate serde;
    extern crate serde_json;
    use serde::ser::Serializer;

    pub fn project_cmd<'a>() -> clap_nested::Command<'a, str> {
        clap_nested::Command::new("project")
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
                let path = PathBuf::from("./").join(project_name);
                fs::create_dir(path)?;
                println!(
                    "Project Path: {}",
                    PathBuf::from("./")
                        .join(project_name)
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .blue()
                );

                println!("{}", "Creating directory structure...".green());
                let model_path = PathBuf::from("./").join(project_name).join("models");
                fs::create_dir(model_path)?;

                let mut test_path = PathBuf::from("./").join(project_name).join("tests");
                fs::create_dir(test_path)?;

                let mut viz_path = PathBuf::from("./").join(project_name).join("visualizations");
                fs::create_dir(viz_path)?;

                let mut output_path = PathBuf::from("./").join(project_name).join("output");
                fs::create_dir(output_path)?;

                let mut data_path = PathBuf::from("./").join(project_name).join("data");
                fs::create_dir(data_path)?;

                println!("{}", "Creating config and entrypoint files...".green());

                let mut full_data_path = env::current_dir().unwrap();
                full_data_path.push("data");

                let mut full_model_path = env::current_dir().unwrap();
                full_model_path.push("models");

                let mut config_file =
                    fs::File::create(PathBuf::from("./").join(project_name).join("sp.json"))?;
                config_file.write_all(
                    config_text(
                        project_name,
                        &full_data_path
                            .to_string_lossy(),
                        &full_model_path
                            .to_string_lossy(),
                    )
                    .as_bytes(),
                )?;

                let mut entrypoint =
                    fs::File::create(PathBuf::from("./").join(project_name).join("index.py"))?;
                entrypoint.write_all(entrypoint_text().as_bytes());

                let init_file =
                    fs::File::create(PathBuf::from("./").join(project_name).join("__init__.py"))?;

                let model_init_file = 
                    fs::File::create(PathBuf::from("./").join(project_name).join("models").join("__init__.py"))?;

                let viz_init_file = 
                    fs::File::create(PathBuf::from("./").join(project_name).join("visualizations").join("__init__.py"))?;

                let mut req_txt = fs::File::create(
                    PathBuf::from("./")
                        .join(project_name)
                        .join("requirements.txt"),
                )?;
                req_txt.write_all(requirements_text().as_bytes());

                println!(
                    "🍻  {} 🍻",
                    "Finished! Boilerplate project created".cyan()
                );
                Ok(())
            })
    }

    pub fn config_text(
        project_name: &str,
        data_path: &str,
        model_path: &str,
    ) -> std::string::String {
        let get_python = std::process::Command::new("which")
            .arg("python")
            .output()
            .expect("Couldn't find python");

        let python_path = get_python.stdout;
        println!("Python found at: {}", String::from_utf8_lossy(&python_path));

        let config_string = json!({
            "project":project_name,
            "entrypoint":"index.py",
            "dependencies":[],
            "python_interpreter":String::from_utf8_lossy(&python_path),
            "models":{
                "path":model_path,
                "classes":[]
            },
            "data":{
                "path":data_path,
                "db":"",
                "dataframe":[]
            }
        });

        return serde_json::to_string_pretty(&config_string).unwrap();
    }

    pub fn entrypoint_text() -> std::string::String {
        return String::from(
            "import json
import csv
import pandas

class Index:
    def __init__(self):
        with open(\"./sp.json\", \"r\") as configs:
            self.config = json.load(configs)
        
        self.data = {}
        self.load_data()
        self.load_models()

    def load_data(self):
        for dataset in self.config[\"data\"][\"dataframe\"]:
            if dataset[\"type\"] == \"csv\":
                self.data[dataset[\"name\"]] = pandas.read_csv(dataset[\"path\"])[
                    [*dataset[\"vars\"]]
                ]
                if dataset[\"datetime\"]:
                    self.data[dataset[\"name\"]][
                        dataset[\"datetime\"]
                    ] = pandas.to_datetime(
                        self.data[dataset[\"name\"]][dataset[\"datetime\"]]


project = Index()",
        );
    }

    pub fn setup_text(
        package_name: &str,
        author: &str,
        email: &str,
        url: &str,
        description: &str,
    ) -> std::string::String {
        return format!(
            "from setuptools import setup

setup(
    name=\"{package_name}\",
    version=\"0.1\",
    description=\"{description}\",
    url=\"{url}\",
    author=\"{author}\",
    author_email=\"{email}\",
    license=\"\",
    packages=[\"{package_name}\"],
    zip_safe=False,
)",
            package_name = package_name,
            description = description,
            url = url,
            author = author,
            email = email
        );
    }

    pub fn requirements_text() -> std::string::String {
        return String::from(
            "atomicwrites==1.2.1
attrs==19.1.0
bokeh==2.0.1
certifi==2019.6.16
cffi==1.12.3
chardet==3.0.4
Click==7.0
docker==4.0.2
idna==2.8
Jinja2==2.11.1
joblib==0.13.2
MarkupSafe==1.1.1
more-itertools==4.3.0
numpy==1.18.2
packaging==20.3
pandas==0.25.0
patsy==0.5.1
Pillow==7.0.0
pip-tools==4.0.0
pluggy==0.8.0
py==1.7.0
pycparser==2.19
pyparsing==2.4.6
pytest==4.0.1
python-dateutil==2.8.1
pytz==2019.2
PyYAML==5.3.1
requests==2.22.0
scikit-learn==0.21.3
scipy==1.3.1
selenium==3.141.0
six==1.14.0
splinter==0.10.0
statsmodels==0.10.1
tornado==6.0.4
typing-extensions==3.7.4.1
urllib3==1.25.3
websocket-client==0.56.0
zstandard==0.11.1
",
        );
    }
}
