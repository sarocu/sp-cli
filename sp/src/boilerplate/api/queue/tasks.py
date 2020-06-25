from __future__ import absolute_import, unicode_literals
import logging

from api.default_queue import queue

log = logging.getLogger("gunicorn.error")


@queue.task
def say_hello(name):
    msg = "Hi {} 👋 ".format(name)
    log.info(msg)
    return msg
