FROM rust:1.45

WORKDIR /usr/src/rust-playground
COPY . .

RUN cargo install --path .
EXPOSE 3030
CMD ["rust-playground"]


