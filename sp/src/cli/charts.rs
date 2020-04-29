pub mod viz {
    use std;
    use std::collections::HashMap;
    use std::env;
    use std::fs;
    use std::io::Write;

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
                let select_chart = Select::new()
                    .with_prompt(&format!("{}", "Select a chart type to add:".cyan()))
                    .item("Basic Bar Chart")
                    .item("Basic Line Chart")
                    .item("Histogram")
                    .item("Scatter Plot")
                    .item("Boxplot")
                    .item("Timeseries Chart")
                    .interact()?;

                let dispatch = {
                    let mut chart_type: HashMap<usize, (String, String)> = HashMap::new();
                    chart_type.insert(0, bar_charts());
                    chart_type.insert(1, simple_line());
                    chart_type
                };

                let (file_contents, file_name) = &dispatch[&select_chart];
                // let file_contents = &dispatch[chart_type].0;
                // let file_name = &dispatch[chart_type].1;
                let mut file_path = env::current_dir().unwrap();
                file_path.push("visualizations");
                file_path.push(file_name);
                let mut viz_file = fs::File::create(file_path)?;
                viz_file.write_all(file_contents.as_bytes());
                Ok(())
            })
    }

    pub fn simple_line() -> (std::string::String, std::string::String) {
        let line_bytes = include_bytes!("../boilerplate/simple_line.py");
        let contents = String::from_utf8_lossy(line_bytes);
        return (contents.to_string(), String::from("simple_line.py"));
    }

    pub fn bar_charts() -> (std::string::String, std::string::String) {
        let bar_bytes = include_bytes!("../boilerplate/bar_charts.py");
        let contents = String::from_utf8_lossy(bar_bytes);
        return (contents.to_string(), String::from("bar_charts.py"));
    }
}
