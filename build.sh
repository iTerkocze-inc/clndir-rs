cargo build
mv ./target/debug/file_sorter ./clndir
echo "Moving the bin file to /usr/bin"
sudo cp ./clndir /usr/bin/

