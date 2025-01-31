# Hyperstition Market Maker + WAVS example

An example of a Hyperstition Market Maker deployed via a WAVS AVS.

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

# MacOS Docker:
# Docker Engine -> Settings -> Resources -> Network -> 'Enable Host Networking'
# or
# brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect
make start-all
```

### Upload your WAVS Service Manager

Deploy Hyperstition Market contracts.

```bash
forge script script/HyperstitionMarket.s.sol:DeployHyperstitionMarket --rpc-url http://localhost:8545 --broadcast
```

Deploy core Eigen contracts:

```bash
wavs-cli deploy-eigen-core
```

Deploy service manager:

```bash
wavs-cli deploy-eigen-service-manager --service-handler $HYPERSTITION_FACTORY_ADDRESS
```

Set the `SERVICE_MANAGER` environment variable with the address of the service manager.

Launch Hyperstition Market:

```bash
forge script script/HyperstitionMarket.s.sol:LaunchHyperstitionMarket --rpc-url http://localhost:8545 --broadcast
```

Deploy Hyperstition Market resolver AVS:

```bash
wavs-cli deploy-service --trigger eth-contract-event \
  --trigger-event-name $(cast sig-event "NewTrigger(uint64,address,bytes)") \
  --trigger-address $HYPERSTITION_FACTORY_ADDRESS \
  --component ./compiled/hyperstition_market_resolver.wasm \
  --submit-address $SERVICE_MANAGER
```

Buy YES in the Hyperstition Market:

```bash
forge script script/HyperstitionMarket.s.sol:BuyYesHyperstitionMarket --rpc-url http://localhost:8545 --broadcast
```

Trigger the Hyperstition Market resolver AVS:

```bash
forge script script/HyperstitionMarket.s.sol:ResolveHyperstitionMarketTrigger \
    --sig "run()" \
    --rpc-url "http://localhost:8545" \
    --broadcast
```

Redeem YES in the resolved Hyperstition Market:

```bash
forge script script/HyperstitionMarket.s.sol:RedeemHyperstitionMarket --rpc-url http://localhost:8545 --broadcast
```
