# Safe AI Module Demo

A demo where an AVS uses an AI agent to propose transactions to a Safe.

TODO:

- [ ] Need to more reliably parse output from agent
- [ ] Safe module could have some extra safety features like permissions (should be deployed with a guard)

Reading and Resources:

- [Zodiac](https://www.zodiac.wiki/documentation): a bunch of useful extensions to the Safe. If you're looking for examples of extending Safe, Zodiac has a ton of them.
- [Safe Modules](https://docs.safe.global/advanced/smart-account-modules): documentation on Safe Modules, allowing easily extending functionality of a Safe.
- [Safe Guard](https://docs.safe.global/advanced/smart-account-guards): documentation on Safe Guards, allowing for checks on Safe transactions.

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

### Deploy contracts

```bash
forge script script/SafeAIModule.s.sol:DeploySafeAIModule --rpc-url http://localhost:8545 --broadcast

# Load the created addresses into the environment
export WAVS_SAFE_MODULE=$(cat .env | grep WAVS_SAFE_MODULE | tail -1 | cut -d '=' -f 2)
# fish shell:
# set -gx WAVS_SAFE_MODULE (cat .env | grep WAVS_SAFE_MODULE | tail -1 | cut -d '=' -f 2)
```

### Deploy service component

```bash
COMPONENT_FILENAME=dao_agent.wasm SERVICE_TRIGGER_ADDR=$WAVS_SAFE_MODULE SERVICE_SUBMISSION_ADDR=$WAVS_SAFE_MODULE make deploy-service
```

### Trigger the AVS to execute a transaction

```bash
forge script script/SafeAIModule.s.sol:AddTrigger --sig "run(string)" "We should donate 1 ETH to 0xDf3679681B87fAE75CE185e4f01d98b64Ddb64a3." --rpc-url http://localhost:8545 --broadcast
```

### Check the balance

```bash
forge script script/SafeAIModule.s.sol:ViewBalance --rpc-url http://localhost:8545
```

> Notice that the balance now contains the 1 ETH donation. If you don't see anything, watch the Anvil and WAVS logs during the trigger creation above to make sure the transaction is succeeding.
