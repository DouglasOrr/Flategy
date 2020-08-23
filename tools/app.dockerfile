FROM rust:1.45.2

RUN rustup default nightly \
    && rustup component add rustfmt
