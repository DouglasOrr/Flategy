FROM ubuntu:18.04

RUN apt-get update          \
    && apt-get install -qy  \
       clang-6.0            \
       clang-tidy-6.0       \
       cmake                \
       g++-8                \
       git-core             \
       libboost1.65-all-dev \
       python3              \
       python3-dev          \
       python3-pip          \
       wget                 \
    && apt-get clean

RUN git clone --branch v1.10.0 https://github.com/google/flatbuffers.git /tmp/flatbuffers \
    && cd /tmp/flatbuffers \
    && cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release -DFLATBUFFERS_BUILD_TESTS=OFF \
    && make -j4 install \
    && cd /tmp/flatbuffers/python \
    && python3 setup.py install \
    && rm -r /tmp/flatbuffers

COPY . /app
RUN pip3 install -r /app/requirements.txt
