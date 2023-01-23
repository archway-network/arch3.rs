#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

echo ":: Build"
archway build --optimize

pushd ${SCRIPT_DIR}/../contracts/increment

echo ":: Switching network"
archway network -m -e 'local'

echo ""
echo ":: Store"
archway store --from alice --no-confirm --no-verify

echo ""
echo ":: Instantiate"
archway instantiate --no-confirm --from alice --default-label --args '{ "count": 0 }'

# CONTRACT_ADDRESS="$(jq -r '[.developer.deployments[] | select(.type == "instantiate")] | .[0].address' config.json)"

# echo ""
# echo ":: Metadata"
# archway metadata --no-confirm --from alice \
#   --owner-address "${CONTRACT_ADDRESS}" \
#   --rewards-address "${CONTRACT_ADDRESS}"

# echo ""
# echo ":: Increment"
# archway tx --no-confirm --from alice --args '{ "increment": {} }'

echo ""
echo ":: Get proposals"
archway query contract-state smart --args '{ "gov_proposals": {} }'
archway query contract-state smart --args '{ "gov_vote": { "proposal_id": 1, "voter": "archway1qfj2k2al6wvyp7fghtw70exv8dlw2lykutcg2x" } }'

git checkout config.json

popd
