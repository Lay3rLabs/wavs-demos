# NFT + WAVS example
An NFT example where the NFT is minted via an WAVS AVS.

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

Deploy NFT contract.

``` bash
forge script script/DeployNFTWithTrigger.s.sol:DeployNFTWithTrigger --rpc-url http://localhost:8545 --broadcast
```

Deploy core Eigen contracts:

``` bash
wavs-cli deploy-eigen-core
```

Deploy service manager:

``` bash
wavs-cli deploy-eigen-service-manager --service-handler $NFT_ADDRESS
```

Set the `SERVICE_PROVIDER` environment variable with the address of the service manager.

Initialize NFT with service manager:
``` bash
forge script script/DeployNFTWithTrigger.s.sol:InitializeNFTWithTrigger --rpc-url http://localhost:8545 --broadcast
```

Deploy component:
``` bash
wavs-cli deploy-service --trigger eth-contract-event \
  --trigger-event-name $(cast sig-event "NewTrigger(uint64,address,bytes)") \
  --trigger-address $NFT_ADDRESS \
  --component ./compiled/autonomous_artist.wasm \
  --submit-address $SERVICE_PROVIDER \
  --service-config '{"fuelLimit":100000000,"maxGas":5000000,"hostEnvs":[],"kv":[],"workflowId":"default","componentId":"default"}'
```

Make a task:

``` bash
forge script script/DeployNFTWithTrigger.s.sol:TestTrigger \
    --sig "run(string)" \
    "How can I be a great artist?" \
    --rpc-url "http://localhost:8545" \
    --broadcast --gas-limit 100000000
```