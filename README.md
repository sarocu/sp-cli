# sp-cli
The Superplus CLI is here to generate boilerplate statistical learning project assets. This includes generating basic project structure, creating new models, making tests and generating visualizations. 

## Create a new project
```bash
sp new project

# optionally with a cool name:
sp new project --name CoolStuff
```

## Add to path:
After building with `cargo build`, add the `sp` executable to the path for use:
```bash
export PATH="~/sp-cli/sp/target/debug:$PATH"
```