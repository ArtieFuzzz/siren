FROM --platform=amd64 rust:latest

WORKDIR /usr/src/siren

COPY . .

RUN cargo install --path .

CMD [ "siren" ]