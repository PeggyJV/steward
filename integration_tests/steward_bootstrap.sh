set -ex

# import orchestrator Sommelier key
steward --config=/root/steward/config.toml keys cosmos-keys-cmd recover steward-key "$MNEMONIC"

# start steward
steward --config=/root/steward/config.toml cosmos-signer
