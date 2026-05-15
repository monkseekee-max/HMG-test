# HMG Site

Leptos CSR prototype for the HMG official website.

The site defaults to Chinese and includes a Chinese / English toggle in the header.

## Run Locally

```bash
env -u NO_COLOR trunk serve --address 127.0.0.1 --port 1420
```

Open <http://127.0.0.1:1420/>.

## Build

```bash
cargo check --target wasm32-unknown-unknown
env -u NO_COLOR trunk build --release
```

`env -u NO_COLOR` works around Trunk 0.21 parsing `NO_COLOR=1` as an invalid boolean.
