#!/bin/bash
set -e

echo "=== 1. Starte Build des Rust-Projekts ==="
cd ha_tui
cargo build --release
cd ..

echo "=== 2. Pfade und Ordner überprüfen ==="
mkdir -p "$HOME/.config/systemd/user/"
mkdir -p "$HOME/services/rust_client/"

if [[ ":$PATH:" != *":/usr/local/bin:"* ]]; then
  echo "WARNUNG: /usr/local/bin ist nicht in deinem PATH!"
  echo "Füge 'export PATH=\$PATH:/usr/local/bin' zu deiner .bashrc/.zshrc hinzu."
else
  echo "Erfolg: Du kannst die TUI jetzt mit dem Befehl 'ha-tui' im Terminal starten."
fi

echo "=== 3. Laufenden 'rust_client' beenden ==="
systemctl --user stop rust_client.service || true

echo "=== 4. Dateien kopieren ==="
cp ./install/rust_client.service "$HOME/.config/systemd/user/"
cp ./ha_tui/target/release/daemon "$HOME/services/rust_client/"
sudo cp ./ha_tui/target/release/tui /usr/local/bin/ha-tui

echo "=== 5. 'rust_client' starten (systemd)"
systemctl --user daemon-reload
systemctl --user enable rust_client.service
systemctl --user start rust_client.service

echo "=== Finish ==="
