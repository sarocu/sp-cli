from __future__ import absolute_import, unicode_literals
from celery import Celery
from dotenv import load_dotenv, find_dotenv
import os

load_dotenv(find_dotenv())

broker = os.environ.get("CELERY_BROKER")
backend = os.eniron.get("CELERY_BACKEND")

queue = Celery("api", broker=broker, backend=backend, include=["api.tasks"])

if __name__ == "__main__":
    queue.start()
