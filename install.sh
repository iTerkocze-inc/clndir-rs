cargo build --release
echo "Moving the bin file to /usr/bin"
sudo mv ./target/release/clndir /usr/bin/clndir

