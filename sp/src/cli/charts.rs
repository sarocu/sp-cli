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

    pub fn simple_line() -> std::string::String {
        return String::from("from bokeh.plotting import figure, output_file, show
from bokeh.models import ColumnDataSource
import pandas


class SimpleLineChart:
    def __init__(
        self, title, xlabel, ylabel, save_as=\"output/line.html\", date_axis=False
    ):
        self.title = title
        self.source = None

        if date_axis:
            self.viz = figure(
                title=title,
                x_axis_label=xlabel,
                y_axis_label=ylabel,
                x_axis_type=\"datetime\",
            )
        else:
            self.viz = figure(title=title, x_axis_label=xlabel, y_axis_label=ylabel)

        output_file(save_as)

    def set_data(self, dataframe):
        self.source = ColumnDataSource(data=dataframe)
        return self

    def add_line(self, xvar, yvar, **kwargs):
        print(kwargs)
        if \"marker\" in kwargs:
            pass
        else:
            self.viz.line(xvar, yvar, source=self.source, **kwargs)

        return self

    def render(self):
        show(self.viz)
");
    }
}
