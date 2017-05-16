# Poner en producción

*No funciona si eres root, pero vale para aprender*

sudo erases the environment variables that you've set, -E preserves them

ROCKET_ENV=production ROCKET_PORT=80 sudo -E /home/freinn/.cargo/bin/cargo run

*Sí funciona*

Ejecutamos esto como root *dicho por sergio*:

ROCKET_ENV=production cargo run --release

# Todo esto se tiene que hacer como root

Abrimos el puerto 80 con el firewall sencillo:

ufw allow 80

# Firewall fácil (UWF)

https://www.digitalocean.com/community/tutorials/how-to-set-up-a-firewall-with-ufw-on-ubuntu-16-04

## Comandos

sudo apt install ufw

ufw enable

ufw allow 22   --> o si no no podremos conectarnos más por ssh! xDDD
ufw allow 80   --> http
ufw allow 443  --> https

## Saber qué programa escucha en un puerto

Para el puerto 80 escribimos:

netstat -tulpn | grep --color :80
