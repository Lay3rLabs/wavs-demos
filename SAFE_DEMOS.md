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

Deploy safe.

``` bash
forge script ./script/SafeModule.s.sol --sig "deployContracts()" --rpc-url http://localhost:8545 --broadcast
```

Deploy core Eigen contracts:

``` bash
wavs-cli deploy-eigen-core
```

Deploy service manager:

``` bash
wavs-cli deploy-eigen-service-manager --service-handler $WAVS_SAFE_MODULE
```

Initialize safe with service manager:

``` bash
forge script ./script/SafeModule.s.sol --sig "initializeModule()" -vvvv --rpc-url http://localhost:8545 --broadcast
```

Deploy component:

``` bash
wavs-cli deploy-service --trigger eth-contract-event \               
  --trigger-event-name $(cast sig-event "NewTrigger(bytes)") \
  --trigger-address <wavs_safe_module> \                       
  --component ./compiled/dao_agent.wasm \
  --submit-address <service_manager>
```

Make a task:

``` bash
# First encode your trigger data
TRIGGER_DATA=$(cast abi-encode "f(string)" "We should donate 1 ETH to 0xDf3679681B87fAE75CE185e4f01d98b64Ddb64a3.")

# Then run the forge script
forge script script/Trigger.s.sol \
    --sig "addTrigger(bytes)" \
    $TRIGGER_DATA \
    --rpc-url "http://localhost:8545" \
    --broadcast
```


### Upload your WAVS Service Manager

```bash
# Deploy Safe
forge script ./script/SafeModule.s.sol --sig "deployContracts()" --rpc-url http://localhost:8545 --broadcast

# Get environment variables
export CLI_EIGEN_CORE_DELEGATION_MANAGER=`docker exec -it wavs bash -c 'jq -r .eigen_core.local.delegation_manager ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_REWARDS_COORDINATOR=`docker exec -it wavs bash -c  'jq -r .eigen_core.local.rewards_coordinator ~/wavs/cli/deployments.json' | tr -d '\r'`
export CLI_EIGEN_CORE_AVS_DIRECTORY=`docker exec -it wavs bash -c  'jq -r .eigen_core.local.avs_directory ~/wavs/cli/deployments.json' | tr -d '\r'`
export FOUNDRY_ANVIL_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80


# Deploy (override: FOUNDRY_ANVIL_PRIVATE_KEY)
forge script ./lib/WAVS/contracts/solidity/scripts/LayerServiceManager.s.sol --rpc-url http://localhost:8545 --broadcast

# TODO this might be broken, but service manger needs to be set from the output
# Grab deployed service manager from script file output
export SERVICE_MANAGER_ADDRESS=`jq -r '.service_manager' "./.docker/cli/script_deploy.json"`
echo "Service Manager Address: $SERVICE_MANAGER_ADDRESS"

# Initialize Safe
forge script ./script/SafeModule.s.sol --sig "initializeModule()" -vvvv --rpc-url http://localhost:8545 --broadcast
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

service_info=`wavs-cli deploy-service --log-level=error --data ./.docker/cli --component $(pwd)/compiled/dao_agent.wasm \
  --trigger-event-name ${trigger_event:2} \
  --trigger eth-contract-event \
  --submit-address ${SERVICE_MANAGER_ADDRESS} \
  --service-config '{"fuelLimit":100000000,"maxGas":5000000,"hostEnvs":[],"kv":[],"workflowId":"default","componentId":"default"}'`

echo "Service info: $service_info"

# Submit AVS request -> chain
SERVICE_ID=`echo $service_info | jq -r .service[0]`; echo "Service ID: $SERVICE_ID"
wavs-cli add-task --input "Nashville,TN" --data ./.docker/cli --service-id ${SERVICE_ID}

# Trigger with forge script
forge script ./script/Trigger.s.sol --sig "addTrigger(string)" "We should donate 1 ETH to 0xDf3679681B87fAE75CE185e4f01d98b64Ddb64a3." -vvvv --broadcast --rpc-url http://localhost:8545

# First encode your trigger data
TRIGGER_DATA=$(cast abi-encode "f(string)" "test")

# Then run the forge script
forge script script/Trigger.s.sol \
    --sig "addTrigger(bytes)" \
    $TRIGGER_DATA \
    --rpc-url "http://localhost:8545" \
    --broadcast

# Grab data from the contract directly
hex_bytes=$(cast decode-abi "getData(uint64)(bytes)" `cast call ${SERVICE_MANAGER_ADDRESS} "getData(uint64)" 1`)
echo `cast --to-ascii $hex_bytes`
```
