# Prediction Market + WAVS Oracle example

An example of a prediction market whose resolution is triggered via an AVS
oracle on WAVS.

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

Deploy prediction market contracts.

```bash
forge script script/PredictionMarket.s.sol:DeployPredictionMarket --rpc-url http://localhost:8545 --broadcast
```

Reload environment variables from .env file:

```bash
source .env
```

Deploy core Eigen contracts:

```bash
wavs-cli deploy-eigen-core
```

Deploy service manager:

```bash
wavs-cli deploy-eigen-service-manager --service-handler $PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS
```

Set the `SERVICE_MANAGER` environment variable with the address of the service manager.

Reload environment variables from .env file:

```bash
source .env
```

Launch prediction market:

```bash
forge script script/PredictionMarket.s.sol:LaunchPredictionMarket --rpc-url http://localhost:8545 --broadcast
```

Reload environment variables from .env file:

```bash
source .env
```

Deploy prediction market oracle AVS that will resolve the market:

```bash
wavs-cli deploy-service --trigger eth-contract-event --trigger-event-name $(cast sig-event "ResolveMarket(uint64,address,bytes)" | cut -c 3-) --trigger-address $PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS --component ./compiled/prediction_market_oracle.wasm --submit-address $SERVICE_MANAGER
```

Buy YES in the Prediction Market:

```bash
forge script script/PredictionMarket.s.sol:BuyYesPredictionMarket --rpc-url http://localhost:8545 --broadcast
```

Trigger the Prediction Market Oracle AVS that resolves the market:

```bash
forge script script/PredictionMarket.s.sol:ResolvePredictionMarketTrigger --sig "run()" --rpc-url http://localhost:8545 --broadcast
```

Redeem YES in the resolved Prediction Market:

```bash
forge script script/PredictionMarket.s.sol:RedeemPredictionMarket --rpc-url http://localhost:8545 --broadcast
```
