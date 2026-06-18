#!/bin/bash

cd ha_tui
cargo build --release
cd ..

# systemd.service kopieren
mkdir -p $USER/.config/systemd/user/
cp ./install/rust_client.service $USER/.config/systemd/user/
# daemon kopieren
mkdir -p $USER/service/rust_client/
cp ./ha_tui/target/release/daemon $USER/service/rust_client/
# service aktivieren
systemd --user enable rust_client
systemd --user start rust_client

# tui kopieren

# PATH aktualisieren
#
