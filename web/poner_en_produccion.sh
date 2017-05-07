#!/bin/bash

# source ~/.bashrc

# sudo erases the environment variables that you've set
# -E preserves them

ROCKET_ENV=production ROCKET_PORT=80 sudo -E /home/freinn/.cargo/bin/cargo run
