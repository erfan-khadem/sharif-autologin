# sharif-autologin
Linux autologin daemon for captive-portal networks

# Installation:
Take a look at `install.sh`
```
cargo build --release
```

Then start another terminal as root:
```
cp target/release/sharif-wifi-autologin /usr/local/bin/

mkdir /etc/sharif-wifi
mkdir /etc/systemd/user

cp config.json /etc/sharif-wifi/
cp sharif-autologin.service /etc/systemd/user

systemctl daemon-reload
```

Now change `/etc/sharif-wifi/config.json` and add your own credentials there

After that you may start the service like this:
```
systemctl --user enable --now sharif-autologin.service
```
