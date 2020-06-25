FROM ubuntu:18.04
RUN apt-get update && apt-get install -y software-properties-common curl libpq-dev build-essential libatlas-base-dev libssl-dev libffi-dev gfortran
RUN add-apt-repository ppa:deadsnakes/ppa -y

RUN apt-get update \
  && apt-get install -y python3.8-dev python3.8-distutils

RUN curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py
RUN python3.8 get-pip.py

ENV PYTHONUNBUFFERED 1
RUN mkdir /code
WORKDIR /code
RUN mkdir /code/api
COPY requirements.txt /code/
RUN pip3.8 install -r requirements.txt

COPY api/* /code/api/
COPY setup.py /code/
RUN pip3.8 install -e /code/.

RUN mkdir /gunicorn
COPY gunicorn/* /gunicorn/

RUN mkdir /code/uploads