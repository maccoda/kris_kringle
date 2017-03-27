#/bin/bash

read -p "Gmail user name: " GMAIL_USERNAME
read -s -p "Gmail P/W: " GMAIL_PASSWORD

export GMAIL_USERNAME=$GMAIL_USERNAME
export GMAIL_PASSWORD=$GMAIL_PASSWORD

cargo run --bin main -- tests/resources/full.toml
