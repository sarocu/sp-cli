pub mod api_ops {
    use std;
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    use clap::ArgMatches;
    use clap_nested;
    use colored::Colorize;

    use dialoguer::Confirmation;

    pub fn api_cmd<'a>() -> clap_nested::Command<'a, str> {
        clap_nested::Command::new("api")
            .description("Add boilerplate for a basic Flask API")
            .runner(|args: &str, matches: &ArgMatches<'_>| {
                println!("{}", "🤖  Creating new Flask REST API boilerplate".cyan());
                let add_gunicorn = Confirmation::new()
                    .with_text("Add Gunicorn as application server?")
                    .interact()?;
                let add_docker = Confirmation::new()
                    .with_text("Add a Dockerfile and Docker-Compose assets?")
                    .interact()?;
                let add_queue = Confirmation::new()
                    .with_text("Add a Celery queue service?")
                    .interact()?;

                let api_path = PathBuf::from("./").join("api");
                fs::create_dir(api_path)?;

                let mut req_txt =
                    fs::File::create(PathBuf::from("./").join("requirements.api.txt"))?;
                req_txt.write_all(get_api_reqs().as_bytes());

                let mut app_text =
                    fs::File::create(PathBuf::from("./").join("api").join("app.py"))?;
                app_text.write_all(get_app().as_bytes());

                let mut env_text =
                    fs::File::create(PathBuf::from("./").join("api").join(".sample.env"))?;
                env_text.write_all(get_api_env().as_bytes());

                let mut dec_text =
                    fs::File::create(PathBuf::from("./").join("api").join("decorators.py"))?;
                dec_text.write_all(get_decorators().as_bytes());

                let init_file =
                    fs::File::create(PathBuf::from("./").join("api").join("__init__.py"))?;

                if add_gunicorn {
                    println!("{}", "Adding Gunicorn assets 🦄".green());
                    let gunicorn_path = PathBuf::from("./").join("gunicorn");
                    fs::create_dir(gunicorn_path)?;
                    let mut supervisor_conf = fs::File::create(
                        PathBuf::from("./")
                            .join("gunicorn")
                            .join("supervisord.conf"),
                    )?;
                    supervisor_conf.write_all(get_supervisor().as_bytes());
                }

                if add_docker {
                    println!("{}", "Adding Docker assets 🚢  📦".cyan());
                    let mut docker_text = fs::File::create(PathBuf::from("./").join("Dockerfile"))?;
                    docker_text.write_all(get_dockerfile().as_bytes());

                    let mut compose_text =
                        fs::File::create(PathBuf::from("./").join("docker-compose.yml"))?;
                    compose_text.write_all(get_compose().as_bytes());
                }

                if add_queue {
                    println!("{}", "Adding Celery and Redis  🧱  assets".cyan());
                    let mut celery_text =
                        fs::File::create(PathBuf::from("./").join("api").join("celery.py"))?;
                    celery_text.write_all(get_celery().as_bytes());

                    let mut task_text =
                        fs::File::create(PathBuf::from("./").join("api").join("tasks.py"))?;
                    task_text.write_all(get_tasks().as_bytes());

                    if add_docker {
                        let mut compose_text =
                            fs::File::create(PathBuf::from("./").join("docker-compose.yml"))?;
                        compose_text.write_all(get_celery_compose().as_bytes());
                    }

                    if add_gunicorn {
                        let mut gunicorn = fs::File::create(
                            PathBuf::from("./")
                                .join("gunicorn")
                                .join("supervisord.conf"),
                        )?;
                        gunicorn.write_all(get_celery_supervisor().as_bytes());
                    }
                }

                Ok(())
            })
    }

    pub fn get_celery_supervisor() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/queue/supervisord.conf");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_celery_compose() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/queue/docker-compose.yml");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_tasks() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/queue/tasks.py");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_celery() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/queue/celery.py");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_compose() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/docker-compose.yml");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_dockerfile() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/Dockerfile");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_supervisor() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/supervisord.conf");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_decorators() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/decorators.py");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_app() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/app.py");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_api_reqs() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/requirements.txt");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }

    pub fn get_api_env() -> std::string::String {
        let get_bytes = include_bytes!("../boilerplate/api/.sample.env");
        let contents = String::from_utf8_lossy(get_bytes);
        return contents.to_string();
    }
}
