from __future__ import absolute_import, unicode_literals

from .celery import queue


@queue.task
def say_hello(name):
    return "Hi {}".format(name)
