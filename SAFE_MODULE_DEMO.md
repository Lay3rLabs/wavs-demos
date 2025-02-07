# Safe Module + WAVS Demo

Status: _highly experimental and fun_

TODO:
- [ ] Need to more reliably parse output from agent
- [ ] Safe module could have some extra safety features like permissions (should be deployed with a guard)
- [ ] A simple predicate function component

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

### Setup contracts

Deploy safe + custom module.

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

### Upload your WAVS Service Manager

Deploy Safe + custom module:
```bash
forge script DeploySafeModule --rpc-url http://localhost:8545 --broadcast
```
Deploy core Eigen contracts:

``` bash
wavs-cli deploy-eigen-core
```

Deploy service manager:

``` bash
source .env
wavs-cli deploy-eigen-service-manager --service-handler $WAVS_SAFE_MODULE
```

Set the `SERVICE_MANAGER_ADDRESS` environment variable with the address of the service manager.

Initialize Safe Module with service manager:
```bash
forge script InitializeSafeModule --rpc-url http://localhost:8545 --broadcast
```

### Build WASI components

> Install `cargo binstall cargo-component` if you have not already. -- https://github.com/bytecodealliance/cargo-component#installation

```bash
make wasi-build
```

## Deploy Service and Verify
Deploy the service:
```bash
wavs-cli deploy-service --trigger eth-contract-event \
  --trigger-event-name $(cast sig-event "NewTrigger(bytes)") \
  --trigger-address $WAVS_SAFE_MODULE \
  --component ./compiled/autonomous_artist.wasm \
  --submit-address $SERVICE_MANAGER_ADDRESS \
  --service-config '{"fuelLimit":100000000,"maxGas":5000000,"hostEnvs":[],"kv":[],"workflowId":"default","componentId":"default"}'
```

Test the service:
```bash
forge script script/SafeModule.s.sol:AddTrigger --sig "run(string)" "We should donate 1 ETH to 0xDf3679681B87fAE75CE185e4f01d98b64Ddb64a3." --rpc-url http://localhost:8545 --broadcast
```
