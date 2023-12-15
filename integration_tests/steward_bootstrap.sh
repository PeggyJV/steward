set -ex

# import orchestrator Sommelier key
steward --config=/root/steward/config.toml keys cosmos recover steward-key "$MNEMONIC"

# start steward
RUST_LOG="info,steward::cork::cache=debug" steward --config=/root/steward/config.toml start
