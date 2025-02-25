# Prediction Market Demo

A demo where a prediction market is resolved by an AVS oracle.

## Instructions

> Install [`cargo install cargo-component --locked`](https://github.com/bytecodealliance/cargo-component#installation) if you have not already.

```bash
# Install initial dependencies.
make setup

# Build the contracts and WASI components.
make build

# Run the tests.
make test
```

### Start Environment

Start an ethereum node (anvil), the WAVS service, and deploy [eigenlayer](https://www.eigenlayer.xyz/) contracts to the local network.

> [!NOTE]
> If you are running on a Mac with an ARM chip, you will need to do the following:
> - Set up Rosetta: `softwareupdate --install-rosetta`
> - Enable Rosetta (Docker Desktop: Settings -> General -> enable "Use Rosetta for x86_64/amd64 emulation on Apple Silicon")
>
> Configure one of the following network options:
> - Docker Desktop: Settings -> Resources -> Network -> 'Enable Host Networking'
> - `brew install chipmk/tap/docker-mac-net-connect && sudo brew services start chipmk/tap/docker-mac-net-connect`

```bash
# Copy over the .env file.
cp .env.example .env

# Start all services.
make start-all
```

> The `start-all` command must remain running in your terminal. Use another terminal to run other commands.
>
> You can stop the services with `ctrl+c` (you may have to press it twice).

### Deploy prediction market contracts

```bash
forge script script/PredictionMarket.s.sol:DeployPredictionMarket --rpc-url http://localhost:8545 --broadcast

# Load the created addresses into the environment
export PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS=$(cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CREATE" and .contractName=="PredictionMarketOracleController") | .contractAddress')
export PREDICTION_MARKET_FACTORY_ADDRESS=$(cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CREATE" and .contractName=="PredictionMarketOracleController") | .additionalContracts[0].address')
export COLLATERAL_TOKEN_ADDRESS=$(cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CREATE" and .contractName=="ERC20Mintable") | .contractAddress')
export CONDITIONAL_TOKENS_ADDRESS=$(cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CALL" and .contractName=="PredictionMarketFactory" and .function=="createConditionalTokenAndLMSRMarketMaker(string,bytes32,address,uint64,uint256)") | .additionalContracts[0].address')
export MARKET_MAKER_ADDRESS=$(cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CALL" and .contractName=="PredictionMarketFactory" and .function=="createConditionalTokenAndLMSRMarketMaker(string,bytes32,address,uint64,uint256)") | .additionalContracts[1].address')
# fish shell:
# set -gx PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS (cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CREATE" and .contractName=="PredictionMarketOracleController") | .contractAddress')
# set -gx PREDICTION_MARKET_FACTORY_ADDRESS (cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CREATE" and .contractName=="PredictionMarketOracleController") | .additionalContracts[0].address')
# set -gx COLLATERAL_TOKEN_ADDRESS (cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CREATE" and .contractName=="ERC20Mintable") | .contractAddress')
# set -gx CONDITIONAL_TOKENS_ADDRESS (cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CALL" and .contractName=="PredictionMarketFactory" and .function=="createConditionalTokenAndLMSRMarketMaker(string,bytes32,address,uint64,uint256)") | .additionalContracts[0].address')
# set -gx MARKET_MAKER_ADDRESS (cat broadcast/PredictionMarket.s.sol/31337/run-latest.json | jq -r '.transactions[] | select(.transactionType=="CALL" and .contractName=="PredictionMarketFactory" and .function=="createConditionalTokenAndLMSRMarketMaker(string,bytes32,address,uint64,uint256)") | .additionalContracts[1].address')

# Save the created addresses to the .env file
echo "" >> .env
echo "PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS=$PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS" >> .env
echo "PREDICTION_MARKET_FACTORY_ADDRESS=$PREDICTION_MARKET_FACTORY_ADDRESS" >> .env
echo "COLLATERAL_TOKEN_ADDRESS=$COLLATERAL_TOKEN_ADDRESS" >> .env
echo "CONDITIONAL_TOKENS_ADDRESS=$CONDITIONAL_TOKENS_ADDRESS" >> .env
echo "MARKET_MAKER_ADDRESS=$MARKET_MAKER_ADDRESS" >> .env
```

### Deploy service component

```bash
COMPONENT_FILENAME=prediction_market_oracle.wasm SERVICE_TRIGGER_ADDR=$PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS SERVICE_SUBMISSION_ADDR=$PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS make deploy-service
```

### Buy YES in the prediction market

```bash
forge script script/PredictionMarket.s.sol:BuyYesPredictionMarket --rpc-url http://localhost:8545 --broadcast
```

> Notice in the logs that you start with 1e18 collateral tokens, and then purchase 1e18 YES shares for 525090975565627651 (~5.25e17) collateral tokens, leaving 474909024434372349 (~4.75e17) collateral tokens remaining.

### Trigger the prediction market oracle AVS to resolve the market

```bash
forge script script/PredictionMarket.s.sol:TriggerOracleResolvePredictionMarket --sig "run()" --rpc-url http://localhost:8545 --broadcast
```

### Redeem YES in the resolved prediction market

```bash
forge script script/PredictionMarket.s.sol:RedeemPredictionMarket --rpc-url http://localhost:8545 --broadcast
```

> Notice in the logs that you redeem 1e18 outcome (YES) shares for 1e18 collateral tokens, ending up with 1474909024434372349 (~1.47e18) collateral tokens. This is more than you started with since you earned a profit from the market by betting on the correct outcome.
