# CV💼

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
npm i
trunk serve
```

## Docker launch

```bash
docker build -t dsalekseev-cv . && docker run -p 1234:1234 dsalekseev-cv
```
