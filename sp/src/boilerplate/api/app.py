import os
import io
import logging
from flask import Flask, request, send_file
from flask_restx import Api, Resource
from dotenv import load_dotenv, find_dotenv

from weather import solar, helpers, constants, api
from decorators import require_key

load_dotenv(find_dotenv())

app = Flask(__name__)
rest = Api(app)

debug = os.environ.get("DEBUG")

if __name__ != "__main__":
    log = logging.getLogger("gunicorn.error")
    app.logger.handlers = log.handlers
    app.logger.setLevel(log.level)


class Resource(Resource):
    """
    Require that all resources apply the bearer token authentication
    """

    method_decorators = [require_key]


@rest.route("/")
class HelloWorld(Resource):
    def get(self):
        return "Hello!"
