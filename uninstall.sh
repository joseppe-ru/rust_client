#!/bin/bash
echo "=== Deinstallation gestartet ==="

# 1. Systemd-Dienst stoppen und komplett deaktivieren
echo "Stoppe Systemd-Dienst..."
systemctl --user stop rust_client.service || true
systemctl --user disable rust_client.service || true

# 2. Dateien löschen
echo "Entferne Dateien..."
rm -f "$HOME/.config/systemd/user/rust_client.service"
rm -rf "$HOME/services/rust_client/"
sudo rm -f "/usr/local/bin/ha-tui"

# 3. Systemd-Manager aufräumen
systemctl --user daemon-reload
systemctl --user reset-failed || true

echo "=== System ist wieder sauber! ==="
