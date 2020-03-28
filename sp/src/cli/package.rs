pub mod package_boilerplate {
    use std;
    use std::env;
    use std::fs;
    use std::io::{BufRead, BufReader, Error, Write};
    use std::path::Path;
    use std::path::PathBuf;

    use clap::{App, Arg, ArgMatches, SubCommand};
    use clap_nested;

    use colored::Colorize;

    extern crate serde;
    extern crate serde_json;
    use serde::ser::Serializer;

    use dialoguer::{Checkboxes, Confirmation, Input};

    pub fn package_cmd<'a>() -> clap_nested::Command<'a, str> {
        clap_nested::Command::new("package")
        .description("Create a new Python package for your data science projects")
        .runner(|args, matches| {
            println!("{}", "⚗️  Create a new Python Package ⚗️".cyan());
            let package_name = Input::<String>::new().with_prompt("Enter a name for your package: ").interact()?;
            let author = Input::<String>::new().with_prompt("Enter an author name: ").interact()?;
            let description = Input::<String>::new().with_prompt("Enter a project description: ").interact()?;

            if Confirmation::new().with_text("Use Python project defaults? 🐍 ").interact()? {
                println!("{}", "Cool, scaffolding now!".cyan());
            } else {
                println!("{}", "Ok, just a few more questions");
                let interpreter = Checkboxes::new().item("Python").item("R").item("Both").with_prompt(&format!("{}", "What kind of interpreter are you using? (hint: use space bar to select)".blue())).interact()?;
                let use_reqs = Confirmation::new().with_text("Use standard scientific requirements.txt?").interact()?;
                let make_api = Confirmation::new().with_text("Create a basic REST API with Flask for the project?").interact()?;
                let pre_commit = Confirmation::new().with_text("Use Git pre-commit hooks for best practices?").interact()?;
                let add_readme = Confirmation::new().with_text("Add a basic README.md?").interact()?;
            }
            Ok(())
        })
    }
}
