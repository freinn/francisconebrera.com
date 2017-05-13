# Poner en producción

*No funciona, pero vale para aprender*

sudo erases the environment variables that you've set, -E preserves them

ROCKET_ENV=production ROCKET_PORT=80 sudo -E /home/freinn/.cargo/bin/cargo run

# Todo esto se tiene que hacer como root

Abrimos el puerto 80 con el firewall sencillo:

ufw allow 80

Ejecutamos esto como root *dicho por sergio*:

ROCKET_ENV=prod cargo run --release

http://wiki.nginx.org/Pitfalls
http://wiki.nginx.org/QuickStart
http://wiki.nginx.org/Configuration
