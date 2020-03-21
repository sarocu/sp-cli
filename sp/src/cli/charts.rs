pub mod viz {
    use std;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    use std::env;
    use std::io::{Write, BufReader, BufRead, Error};

    use clap_nested;
    use clap::{Arg, App, SubCommand, ArgMatches};

    use colored:: Colorize;

    extern crate serde_json;
    extern crate serde;
    use serde::ser::Serializer;

    use dialoguer::{Input, Confirmation, Checkboxes, Select};

    pub fn chart_cmd<'a>() -> clap_nested::Command<'a, str> {
        clap_nested::Command::new("viz")
        .description("Add a new vizualization using Bokeh to an existing Superplus Project")
        .runner(|args, matches| {
            println!("{}", "📈   Create a Bokeh Vizualization 📊".cyan());
            let chart_type = Select::new()
                .with_prompt(&format!("{}","Select a chart type to add:".cyan()))
                .item("Basic Bar Chart")
                .item("Basic Line Chart")
                .item("Histogram")
                .item("Scatter Plot")
                .item("Boxplot")
                .item("Timeseries Chart")
                .interact()?;

            Ok(())
        })
    }
}