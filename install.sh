#!/bin/bash

echo "generating binary..."
cargo build --release
echo "installing rusty-cowsay..."
sudo cp ./target/release/rusty-cowsay /usr/bin/
echo "installing completion file"
sudo cp ./rusty-cowsay.bash /usr/share/bash-completion/completions/
rusty-cowsay -W18 Install finished! Enjoy your cowsay!