#!/bin/bash

# source ~/.bashrc

# sudo erases the environment variables that you've set
# -E preserves them

# Todo esto se tiene que hacer como root

Abrimos el puerto 80 con el firewall sencillo:

ufw allow 80

Ejecutamos esto como root:

ROCKET_ENV=prod cargo run --release
