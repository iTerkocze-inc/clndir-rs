cargo build --release
echo "Moving the bin file to /usr/bin"
sudo mv ./target/release/clndir /usr/bin/clndir
echo "If you want to configure your program copy the example config file in respository to \"~/.config/clndir/\". This config file has all the default settings set up so the program works as it would without any tweaking."
