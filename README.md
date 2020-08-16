# sp-cli
The Superplus CLI is here to generate boilerplate statistical learning project assets. This includes generating basic project structure, creating new models, making tests and generating visualizations. 

## Building from Source
Build from source using the Rust's `cargo`:
```
cd sp
cargo build
```

Or build optimized releases using Make:
```bash
make clean
make release
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
# output:
📈   Create a Bokeh Vizualization 📊
Select a chart type to add::
  Basic Bar Chart
> Basic Line Chart
  Histogram
  Scatter Plot
  Boxplot
  Timeseries Chart
```

### Model Structure
Superplus are configured as sets of models, visualizations, and data files that have a common entrypoint, `index.py` which you can run with a `sp run start`. Of course you can define any number of scripts to run and add them to `sp.json` or add command line options to `index` to customize your scripts. With this in mind, a Superplus model implements basic functionality and the entrypoint file is expected to perform any data transformations that are necessary. 

### Available Models
| type | description |
| `simple-linear` | Scikit-Learn linear regression |
| `vector-ar` | Statsmodels vectorized autoregressive time-series |
| `pca` | Scikit-Learn principle components |
| `mlpr` | Scikit-Learn nueral net multi-layer perceptron regressor |

### Using Pipenv instead of Pip
When creating a new project, add the `--pipenv` flag to make sp install dependencies and run scripts using pipenv:
```bash
sp new project --name reallycool --pipenv
```
Keep in mind that pipenv will take a few minutes to bootstrap.

## Scaffold out a REST API Project
`sp` can generate boilerplate for a Flask API project, including Docker assets, a Redis queue service, and Gunicorn/Supervisord configs. Just do `sp new api` and follow the prompts. When `sp` is done doing its thing, copy the sample .env file (`.sample.env`) to `./.env` and update the environment variables to include a bearer token for API routes. 

### Docker and Docker-Compose
The Docker configs help you develop production ready services out of the box. The Dockerfile provided uses an Ubuntu image and supports a basic scientific Python environment including Scikit-Learn, SciPy, Pandas, and Statsmodels packages as well as web technologies such as Flask and Celery. After sp bootstraps the API, you can build the images and run the Flask, Celery Worker, and Redis services using:
```bash
# run containers in the background:
docker-compose up -d

# check out logs
docker-compose logs -f worker
```

The Flask service uses Gunicorn as an application server and Supervisor as a process manager. Logs are configured for you, check out `./gunicorn/gunicorn.error.log` for more information about how your app is running. The Docker image created for Flask and the Celery Worker can be directly deployed to container services such as AWS Fargate or Azure Container Instances.

### API Bearer Auth
By default, all routes are protected by bearer tokens using a simple decorator. You should either generate a bearer token and include it in your `.env` file and in the authorization header of any requests you make for your API, or you should remove decorator altogether. The boilerplate adds the decorator to all Flask Resources with the following in `app.py`:
```python
class Resource(Resource):
    """
    Require that all resources apply the bearer token authentication
    """

    method_decorators = [require_key]
```
If you remove the `require_key` decorator from the array, the bearer token will no longer be required.


### Task Queue
The API boilerplate includes a simple queue service `default_queue.py` using Redis. If you're using Docker, a basic Redis service is included in the `docker-compose.yaml`. The task queue lets you kick expensive or long running tasks to the background using a `delay()` method, demonstrated in the POST method for the `/hello` route. 

You probably don't want to wait on the job to complete before returning, instead you should use a database or save output to the file system and return an ID or path that lets users retrieve results later.


## Command Options
Superplus CLI generates code for a number of applications including microservice APIs, queues, machine learning and data science projects

| Command | Subcommand | Option | Description |
|---------|------------|--------|-------------|
| new | project | `-n` or `--name` | Name of the project and directory |
| new | csv | `-p` or `--path` | Path to a CSV file you want to include in the project |
| new | viz | N/A | Kicks off the new visualization workflow |
| new | model | `-t` or `--type` | type of model you'd like to boilerplate for |
| new | api | N/A | Kicks off the  new Flask API workflow |
| run | script name | N/A | Run a script defined in `sp.json`, similar to `npm run ...` | 

## Add to path:
After building with `cargo build`, add the `sp` executable to the path for use:
```bash
export PATH="~/sp-cli/sp/target/debug:$PATH"
```