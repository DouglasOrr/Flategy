FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
    && apt-get install -y \
    cmake \
    locales \
    python3 \
    python3-pip \
    wget \
    && apt-get clean \
    && locale-gen en_GB.UTF-8

RUN useradd -m jovyan
USER jovyan

RUN wget -nv -O - https://sh.rustup.rs | sh -s -- -y

ENV LC_ALL=en_GB.UTF-8 \
    LANG=en_GB.UTF-8 \
    LANGUAGE=en_GB.UTF-8 \
    PATH="/home/jovyan/.cargo/bin:${PATH}"

RUN cargo install \
    evcxr_jupyter \
    && evcxr_jupyter --install

RUN pip3 install \
    black \
    flake8 \
    jupyterlab \
    matplotlib \
    numpy \
    pandas \
    pylint \
    pytest \
    PyYAML

ENV PATH="/home/jovyan/.local/bin:${PATH}"
