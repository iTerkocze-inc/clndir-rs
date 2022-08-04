# CLNDIR - A directory cleaner written in Rust  
Clndir is a directory cleaning program written in Rust with many features, configuration options and nice colored output.

## Installation
To build and install clndir all you have to do is clone this repository and run the ./build.sh script that will build the program and move the binary to `/usr/bin/` or do it manually by running these 3 commands:
```bash
cd ./clndir-rs/
cargo build
sudo mv ./target/debug/file_sorter /usr/bin/
```
