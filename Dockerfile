FROM nvidia/cuda:10.0-cudnn7-devel

RUN apt-get update          \
    && apt-get install -qy  \
       ffmpeg               \
       python3              \
       python3-dev          \
       python3-pip          \
    && apt-get clean

COPY requirements.txt /tmp/requirements.txt
RUN pip3 install -r /tmp/requirements.txt \
    && rm /tmp/requirements.txt
