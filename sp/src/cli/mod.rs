pub mod api;
pub mod charts;
pub mod data;
pub mod models;
pub mod package;
pub mod run;
pub mod version;

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

    use dialoguer::Confirmation;

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
                .arg(
                    Arg::with_name("python")
                        .short("p")
                        .long("python")
                        .required(false)
                        .takes_value(true)
                        .value_name("STRING")
                        .help("Set the path for the preferred python interpreter"),
                )
                .arg(
                    Arg::with_name("pipenv")
                        .long("pipenv")
                        .required(false)
                        .takes_value(false)
                        .help("toggles package setup with pipenv instead of virtualenv"),
                )
            })
            .runner(|args: &str, matches: &ArgMatches<'_>| {
                let system_python = || {
                    let get_python = std::process::Command::new("which")
                        .arg("python3")
                        .output()
                        .expect("Couldn't find python");

                    let python_path_cmd = match std::str::from_utf8(&get_python.stdout) {
                        Ok(v) => v,
                        Err(e) => panic!("Could not identify system python: {}", e),
                    };

                    let mut python_path = python_path_cmd.to_owned();

                    if python_path.ends_with('\n') {
                        python_path.pop();
                        if python_path.ends_with('\r') {
                            python_path.pop();
                        }
                    }
                    println!("Python found at: {}", python_path);
                    return python_path;
                };

                let project_name = matches.value_of("name").unwrap_or("app");
                let mut python_path = matches
                    .value_of("python")
                    .unwrap_or(&system_python())
                    .to_owned();
                let use_pipenv = matches.is_present("pipenv"); //.unwrap_or(false);
                println!("{:?}", use_pipenv);
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

                let mut viz_path = PathBuf::from("./")
                    .join(project_name)
                    .join("visualizations");
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

                let mut req_txt = fs::File::create(
                    PathBuf::from("./")
                        .join(project_name)
                        .join("requirements.txt"),
                )?;
                req_txt.write_all(requirements_text().as_bytes());

                let mut python_type = String::from("system");

                if use_pipenv {
                    println!(
                        "{}{}",
                        "Initializing pipenv...".green(),
                        "this could take a minute".bright_magenta().on_black()
                    );
                    let project_path = PathBuf::from("./").join(project_name);

                    let init_pipenv = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(format!(
                            "cd {} && pipenv install",
                            project_path.to_string_lossy()
                        ))
                        .output()
                        .expect("Couldn't perform `pipenv install`");

                    python_path = String::from("pipenv");
                    python_type = String::from("pipenv");
                } else {
                    let add_venv = Confirmation::new()
                        .with_text("Add a new virtual environment?")
                        .interact()?;

                    if add_venv {
                        let make_venv = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(format!(
                                "{py} -m venv {n}/venv",
                                py = python_path,
                                n = project_name
                            ))
                            .output()
                            .expect("Couldn't create virtualenv");

                        python_path = String::from("venv/bin/activate");
                        python_type = String::from("venv");
                    }
                }

                let mut config_file =
                    fs::File::create(PathBuf::from("./").join(project_name).join("sp.json"))?;
                config_file.write_all(
                    config_text(
                        project_name,
                        &full_data_path.to_string_lossy(),
                        &full_model_path.to_string_lossy(),
                        &python_path,
                        &python_type,
                    )
                    .as_bytes(),
                )?;

                let mut entrypoint =
                    fs::File::create(PathBuf::from("./").join(project_name).join("index.py"))?;
                entrypoint.write_all(entrypoint_text().as_bytes());

                let init_file =
                    fs::File::create(PathBuf::from("./").join(project_name).join("__init__.py"))?;

                let model_init_file = fs::File::create(
                    PathBuf::from("./")
                        .join(project_name)
                        .join("models")
                        .join("__init__.py"),
                )?;

                let viz_init_file = fs::File::create(
                    PathBuf::from("./")
                        .join(project_name)
                        .join("visualizations")
                        .join("__init__.py"),
                )?;

                let mut req_txt = fs::File::create(
                    PathBuf::from("./")
                        .join(project_name)
                        .join("requirements.txt"),
                )?;
                req_txt.write_all(requirements_text().as_bytes());

                println!("🍻  {} 🍻", "Finished! Boilerplate project created".cyan());
                Ok(())
            })
    }

    pub fn config_text(
        project_name: &str,
        data_path: &str,
        model_path: &str,
        python_path: &str,
        python_type: &str,
    ) -> std::string::String {
        let config_string = json!({
            "project":project_name,
            "entrypoint":"index.py",
            "scripts": {
                "start":"python index.py",
                "lint":"black ."
            },
            "dependencies":[],
            "python_interpreter":python_path,
            "python_env":python_type,
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
        let index_bytes = include_bytes!("../boilerplate/index.py");
        let contents = String::from_utf8_lossy(index_bytes);
        return contents.to_string();
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
        let req_bytes = include_bytes!("../boilerplate/requirements.txt");
        let contents = String::from_utf8_lossy(req_bytes);
        return contents.to_string();
    }
}
