# Development
## WSL Dev Setup
Requirements:
```bash
sudo apt-get update
sudo apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev
sudo apt-get install -y libxcursor-dev alsa libegl1-mesa libegl1
# if dynamic_linking is enabled in bevy
sudo apt-get install -y lld
```

## Windows Dev Setup
Requirements for `dynamic_linking` in Bevy:
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```