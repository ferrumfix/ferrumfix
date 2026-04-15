#!/usr/bin/env bash

# Kill QuickFIX executor upon exit.
# From <https://stackoverflow.com/a/2173421/5148606>
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

# Clear QuickFIX storage.
rm -rf target/quickfix-store

CONFIG_PATH="examples/20_tokio_fix_initiator/quickfix_executor_config.ini"
executor="./lib/quickfix/config/examples/executor/C++/executor $CONFIG_PATH"

$executor &

cargo run --bin example_tokio_fix_initiator
