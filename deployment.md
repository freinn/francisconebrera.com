# Deployment

## Compilar todo para que vaya lo más rápido posible.

rustup run nightly cargo build --release

ROCKET_ENV=prod cargo run --release
