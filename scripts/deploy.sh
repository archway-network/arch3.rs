#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR="$(dirname "${SCRIPT_DIR}")"

CHAIN_ID="${CHAIN_ID:-constantine-3}"

echo ":: Build"
#archway contracts build

echo ""
echo ":: Switching network"
archway config chains use "${CHAIN_ID}"

echo ""
echo ":: Store"
archway contracts store increment --from deployer --no-confirm

echo ""
echo ":: Instantiate"
archway contracts instantiate increment --from deployer --no-confirm --skip-validation --args '{ "count": 0 }'

CONTRACT_ADDRESS="$(jq -r '[.deployments[] | select(.action == "instantiate")] | .[0].contract.address' "${ROOT_DIR}/.archway/${CHAIN_ID}.json")"

echo ""
echo ":: Metadata"
archway contracts metadata increment --from deployer --no-confirm --owner-address "${CONTRACT_ADDRESS}" --rewards-address "${CONTRACT_ADDRESS}"

echo ""
echo ":: Increment"
archway contracts execute increment --from deployer --no-confirm --skip-validation --args '{ "increment": {} }' --gas-adjustment 1.4

echo ""
echo ":: Get Rewards"
archway rewards query "${CONTRACT_ADDRESS}"
