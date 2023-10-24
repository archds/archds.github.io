FROM node:21-alpine as node-instsaller
WORKDIR /usr/src/app
COPY package-lock.json package.json ./
RUN npm ci


FROM rust:1.73-slim as rust-builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
WORKDIR /usr/src/app
COPY --from=node-instsaller /usr/src/app/node_modules ./node_modules
COPY . .
RUN trunk build --release

FROM node:21-alpine as node-runner
RUN npm install -g http-server
WORKDIR /usr/src/app
COPY --from=rust-builder /usr/src/app/dist ./dist
WORKDIR /usr/src/app/dist
CMD [ "http-server", "-g", "-b", "-p 1234" ]