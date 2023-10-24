# package-lock install
FROM node:21-alpine as node-instsaller

WORKDIR /usr/src/app

COPY package-lock.json package.json ./

RUN npm ci


# Trunk build
FROM rust:1.73-slim as rust-builder

ENV TRUNK_VER="v0.17.5"

RUN apt-get update && apt-get install wget -y
RUN rustup target add wasm32-unknown-unknown
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/${TRUNK_VER}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

WORKDIR /usr/src/app

COPY --from=node-instsaller /usr/src/app/node_modules ./node_modules
COPY . .

RUN ./trunk build --release


# Runner
FROM node:21-alpine as node-runner

RUN npm install -g http-server

WORKDIR /usr/src/app

COPY --from=rust-builder /usr/src/app/dist ./dist

WORKDIR /usr/src/app/dist

CMD [ "http-server", "-g", "-b", "-p 1234" ]