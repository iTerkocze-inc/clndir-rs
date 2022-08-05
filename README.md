# CLNDIR - A directory cleaner written in Rust  
Clndir is a directory cleaning program written in Rust with many features, configuration options and nice colored output.

## Installation
To build and install clndir all you have to do is clone this repository and run the ./install.sh script that will build the program and move the binary to `/usr/bin/` or do it manually by running these 3 commands:
```bash
cd ./clndir-rs/
cargo build
sudo mv ./target/debug/file_sorter /usr/bin/
```
## Roadmap
✔️ is done, 〰️ is in progress and 🕑 is planned
- ✔️ Customizable sorting directories and archive folder's name and path
- ✔️ Cleaning multiple folders
- 〰️ Sorting files based on their name
- 🕑 Multi-threaded sorting
- 🕑 Switching between config files
- 🕑 Sorting files based on their last modification date
- 🕑 Exceptions for files to not move them
- 🕑 Separate sorting directories paths

###### This is still in early development so expect frequent updates. I'm open to any ideas ^^
