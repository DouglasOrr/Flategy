FROM ubuntu:18.04

RUN apt-get update          \
    && apt-get install -qy  \
       clang-6.0            \
       clang-tidy-6.0       \
       g++-8                \
       libboost1.65-all-dev \
       python3              \
       python3-dev          \
       python3-pip          \
       wget                 \
    && apt-get clean

COPY . /app
RUN pip3 install -r /app/requirements.txt
