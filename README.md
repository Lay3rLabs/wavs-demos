# Safe + WAVS

A variety of WASI components that can be leveraged to extend the functionality of the Gnosis Safe with custom Safe Modules and Guards.

Status: _highly experimental and fun_

TODO:
- [ ] Need to more reliably parse output from agent
- [ ] Safe module could have some extra safety features like permissions
- [ ] Eth Cosmos Query example needs to be flushed out
- [ ] Has not yet been tested with WAVS
- [ ] A guard function that reviews the prompt

Reading and Resources:
- [Zodiac](https://www.zodiac.wiki/documentation): a bunch of useful extensions to the Safe. If you're looking for examples of extending Safe, Zodiac has a ton of them.
- [Safe Modules](https://docs.safe.global/advanced/smart-account-modules): documentation on Safe Modules, allowing easily extending functionality of a Safe.
- [Safe Guard](https://docs.safe.global/advanced/smart-account-guards): documentation on Safe Guards, allowing for checks on Safe transactions.

## Installation

Create a new project using this template:

```bash
# If you don't have forge: `curl -L https://foundry.paradigm.xyz | bash`
forge init --template Lay3rLabs/wavs-foundry-template my-wavs
```

### Solidity

```bash
# Initialize the submodule dependencies
forge install

# Build the contracts
forge build

# Run the solidity tests. alias: `make test`
forge test
```

> You can also use `make build` to build the contracts, bindings, and components.

## Rust

```bash
# Generate new bindings from your contract(s)
make bindings

# Run rust tests
make test
```

## WAVS

### Install the WAVS CLI

```bash
# MacOS: if you get permission errors: eval `ssh-agent -s` && ssh-add
(cd lib/WAVS; cargo install --path ./packages/cli)
```

### Start Anvil, WAVS, and Deploy Eigenlayer

```bash
# copy over the .env file
cp .env.example .env

# [!] Get your key from: https://openweathermap.org/
# Update the WAVS_ENV_OPEN_WEATHER_API_KEY in the .env file with your key`

# MacOS Docker:
# Docker Engine -> Settings -> Resources -> Network -> 'Enable Host Networking'
# or
# brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect
make start-all
```

### Upload your WAVS Service Manager

```bash
# Deploy Safe
forge script ./script/SafeModule.s.sol --sig "deployContracts()" --rpc-url http://localhost:8545 --broadcast

# Deploy (override: FOUNDRY_ANVIL_PRIVATE_KEY)
forge script ./lib/WAVS/contracts/solidity/scripts/LayerServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast

# Initialize Safe
forge script ./script/SafeModule.s.sol --sig "initializeModule()" -vvvv --rpc-url http://localhost:8545 --broadcast

# Grab deployed service manager from script file output
export SERVICE_MANAGER_ADDRESS=`jq -r '.service_manager' "./.docker/cli/script_deploy.json"`
echo "Service Manager Address: $SERVICE_MANAGER_ADDRESS"
```

### Build WASI components

> Install `cargo binstall cargo-component` if you have not already. -- https://github.com/bytecodealliance/cargo-component#installation

```bash
make wasi-build

# TODO: currently broken upstream
# Verify execution works as expected without deploying
# wavs-cli exec --component $(pwd)/compiled/eth_trigger_weather.wasm --input Nashville,TN
```

## Deploy Service and Verify

```bash
# add read-write access
sudo chmod 0666 .docker/cli/deployments.json

# Contract trigger function signature to listen for
trigger_event=$(cast sig-event "NewTrigger(bytes)"); echo "Trigger Event: $trigger_event"

service_info=`wavs-cli deploy-service --log-level=error --data ./.docker/cli --component $(pwd)/compiled/eth_trigger_weather.wasm \
  --trigger-event-name ${trigger_event:2} \
  --trigger eth-contract-event \
  --submit-address ${SERVICE_MANAGER_ADDRESS} \
  --service-config '{"fuelLimit":100000000,"maxGas":5000000,"hostEnvs":["WAVS_ENV_OPEN_WEATHER_API_KEY"],"kv":[],"workflowId":"default","componentId":"default"}'`

echo "Service info: $service_info"

# Submit AVS request -> chain
SERVICE_ID=`echo $service_info | jq -r .service[0]`; echo "Service ID: $SERVICE_ID"
wavs-cli add-task --input "Nashville,TN" --data ./.docker/cli --service-id ${SERVICE_ID}

# Grab data from the contract directly
hex_bytes=$(cast decode-abi "getData(uint64)(bytes)" `cast call ${SERVICE_MANAGER_ADDRESS} "getData(uint64)" 1`)
echo `cast --to-ascii $hex_bytes`
```
