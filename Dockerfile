# FROM rust:latest

# RUN mkdir -p /usr/src/app
# WORKDIR /usr/src/app

# COPY . /usr/src/app

# CMD ["cargo", "build", "-q"]

FROM rust:latest

RUN mkdir -p /usr/src/
WORKDIR /usr/src/
RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./tests ./tests

RUN cargo build --release
#RUN rm src/*.rs

COPY ./src ./src

CMD ["cargo", "build", "--release"]