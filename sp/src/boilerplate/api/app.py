import os
import io
import logging
from flask import Flask, request, abort
from flask_restx import Api, Resource
from dotenv import load_dotenv, find_dotenv

from decorators import require_key
from default_queue import queue
from api.tasks import say_hello

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


@rest.route("/hello")
class HelloWorld(Resource):
    def get(self):
        return "Hello!"

    def post(self):
        try:
            name = request.args["name"]
            job = say_hello.delay(name)
            job.wait()
            return {"message": job.get()}
        except Exception as e:
            log.error(e)
            abort(400, "Something went wrong!")
