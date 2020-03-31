# sp-cli
The Superplus CLI is here to generate boilerplate statistical learning project assets. This includes generating basic project structure, creating new models, making tests and generating visualizations. 

## Building the CLI
Build from source using the Rust's `cargo`:
```
cd sp
cargo build
```

## Create a new project
```bash
sp new project

# optionally with a cool name:
sp new project --name CoolStuff

cd CoolStuff

# add csv files to the project:
sp new csv -p ~/Downloads/AGNC.csv

# add a basic regression model:
sp new model --type simple_linear

# add some visualizations:
sp new viz
```

## Command Options
| Command | Subcommand | Option | Description |
|---------|------------|--------|-------------|
| new | project | `-n` or `--name` | Name of the project and directory |
| new | csv | `-p` or `--path` | Path to a CSV file you want to include in the project |
| new | viz | N/A | Kicks off the new visualization workflow |
| new | model | `-t` or `--type` | type of model you'd like to boilerplate for |

## Add to path:
After building with `cargo build`, add the `sp` executable to the path for use:
```bash
export PATH="~/sp-cli/sp/target/debug:$PATH"
```