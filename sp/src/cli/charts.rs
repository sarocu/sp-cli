pub mod viz {
    use std;

    use clap_nested;

    use colored::Colorize;

    extern crate serde;
    extern crate serde_json;

    use dialoguer::Select;

    pub fn chart_cmd<'a>() -> clap_nested::Command<'a, str> {
        clap_nested::Command::new("viz")
            .description("Add a new vizualization using Bokeh to an existing Superplus Project")
            .runner(|args, matches| {
                println!("{}", "📈   Create a Bokeh Vizualization 📊".cyan());
                let chart_type = Select::new()
                    .with_prompt(&format!("{}", "Select a chart type to add:".cyan()))
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
