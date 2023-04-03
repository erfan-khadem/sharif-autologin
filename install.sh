#!/usr/bin/bash

cargo build --release

sudo cp target/release/sharif-wifi-autologin /usr/local/bin/

sudo mkdir -p /etc/sharif-wifi
sudo mkdir -p /etc/systemd/user

if ! [[ -e /etc/sharif-wifi/config.json ]]; then
	sudo cp config.json /etc/sharif-wifi/
fi

sudo cp sharif-autologin.service /etc/systemd/user

sudo systemctl daemon-reload

echo "Don't forget to put your credentials in /etc/sharif-wifi/config.json"
echo "run 'systemctl --user enable --now sharif-autologin.service' to enable the service on startup"

