# Marinade Bond Monitor


# Installation
Install Rust:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`source $HOME/.cargo/env`

Verify
`cargo --version`
`rustc --version`

# Build
`cargo build --release`

Binary location: `target/release/marinade-monitor`

# Environment Variables
Create: `/etc/opt/app/slack/.env`
Contents:
`
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/XXX/XXX/XXX
`

# Running 
Run manually: `./target/release/marinade-monitor`

---

# APIs Used
Validator API: `https://validators-api.marinade.finance/validators?limit=9999&epochs=0`

Bond API: `https://validator-bonds-api.marinade.finance/bonds/institutional`