cargo build
mv ./target/debug/file_sorter ./clndir
cp ./src/* ../../Backup/file_sorter/
echo "Moving the bin file to /usr/bin"
sudo cp ./clndir /usr/bin/

