archwayd config chain-id constantine-2
archwayd config keyring-backend test
archwayd config output json
archwayd config node https://rpc.constantine-2.archway.tech:443
archwayd config broadcast-mode block

# need some uconst on this acc to pay fees
export deployer_name=acc0
export chain_id=constantine-2
export coin=uconst

export deployer_address=$(archwayd keys show -a $deployer_name)
echo "Deployer address: '$deployer_address'"

RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown

# store on testnet
stored_info=$(archwayd tx wasm store "../target/wasm32-unknown-unknown/release/increment.wasm" --from "$deployer_name" --gas auto -b block -y --chain-id $chain_id -o json --fees 10000${coin})
code_id=$(echo $stored_info | jq -r '.logs[0].events[1].attributes[] | select(.key=="code_id") | .value')
echo "Stored code_id: '$code_id'"

# Instantiate
archwayd tx wasm instantiate $code_id '{"count": 1}' --label inc --gas auto --fees 10000${coin} --from "$deployer_name" -b block -y --keyring-backend test --admin $deployer_address > /dev/null
contract_addr=$(archwayd query wasm list-contract-by-code $code_id -o json | jq -r '.contracts[-1]')
echo "Instantiated contract: '$contract_addr'"


echo "Set metadata field owner address of contract intantiated to its own address..."
archwayd tx rewards set-contract-metadata $contract_addr --gas auto --fees 10000${coin} --from $deployer_name -b block -y --keyring-backend test --owner-address $contract_addr > /dev/null

echo "Update contract rewards address from the contract handler to his own address..."
archwayd tx wasm execute $contract_addr '{"update_rewards_address": {}}' --from "$deployer_name" -b block -y --keyring-backend test --gas 200000 --fees 10000${coin} > /dev/null

echo "Execute the handler increment to generate some rewards from gas fees payed..."
archwayd tx wasm execute $contract_addr '{"increment": {}}' --from "$deployer_name" -b block -y --keyring-backend test --gas auto --fees 10000${coin} > /dev/null

echo "Query the reward records of the contract..."
archwayd query rewards rewards-records $contract_addr

echo "Withdraw the rewards from the contract to itself (as the rewards address is itself)..."
archwayd tx wasm execute $contract_addr '{"withdraw_rewards": {}}' --from "$deployer_name" -b block -y --keyring-backend test --gas 200000 --fees 10000${coin}  > /dev/null

echo "Query balance of the contract address... (should have the rewards withdrawn)"
archwayd q bank balances $contract_addr
