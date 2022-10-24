# Browse the docs locall and offline

- rustup component add rust-docs
- rustup doc --std
- cargo doc --open

# Run with proxy server

- trunk serve --port 3000 --proxy-backend=https://yew.rs/tutorial
- RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve --port 3000 --proxy-backend=https://yew.rs/tutorial

# Note with web-sys API

- Add this setting to VSCode:

```json
    "rust-analyzer.server.extraEnv": {
        "RUSTFLAGS": "--cfg=web_sys_unstable_apis"
    }
```
