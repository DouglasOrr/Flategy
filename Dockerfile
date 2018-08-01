FROM ubuntu:18.04

RUN apt-get update         \
    && apt-get install -qy \
       python3             \
       python3-pip         \
    && apt-get clean

COPY . /app
RUN pip3 install -r /app/requirements.txt
