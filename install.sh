#!/usr/bin/bash

cargo build --release
cp target/release/sharif-wifi-autologin /usr/local/bin/

mkdir /etc/sharif-wifi
mkdir /etc/systemd/user

cp config.json /etc/sharif-wifi/
cp sharif-autologin.service /etc/systemd/user

systemctl daemon-reload

echo "Don't forget to put your credentials in /etc/sharif-wifi/config.json"
echo "run 'systemctl --user enable --now sharif-autologin.service' to enable the service on startup"

