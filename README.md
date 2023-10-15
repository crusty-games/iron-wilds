# Development
## WSL Dev Setup
Requirements:
```bash

sudo apt-get update
sudo apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev
sudo apt-get install -y lld libxcursor-dev alsa libegl1-mesa libegl1
```

## Windows Dev Setup
Requirements:
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```