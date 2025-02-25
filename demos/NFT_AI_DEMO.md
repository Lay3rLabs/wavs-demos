# NFT + AI Demo

A demo where an NFT is minted by an AVS with AI-generated metadata.

## Instructions

> Install [`cargo install cargo-component --locked`](https://github.com/bytecodealliance/cargo-component#installation) if you have not already.

> Install [Ollama](https://ollama.com/download) and download the llama3.1 model via `ollama pull llama3.1`.

```bash
# Install initial dependencies.
make setup

# Build the contracts and WASI components.
make build

# Run the tests.
make test
```

### Start Ollama, Anvil, and WAVS

```bash
# Start Ollama in one terminal and leave it running.
ollama run llama3.1
```

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

### Deploy NFT contract

```bash
forge script script/NFTDemo.s.sol:DeployNFTDemo --rpc-url http://localhost:8545 --broadcast

# Load the created NFT address into the environment
export NFT_ADDRESS=$(cat .env | grep NFT_ADDRESS | tail -1 | cut -d '=' -f 2)
```

### Deploy service component

```bash
COMPONENT_FILENAME=autonomous_artist.wasm SERVICE_TRIGGER_ADDR=$NFT_ADDRESS SERVICE_SUBMISSION_ADDR=$NFT_ADDRESS make deploy-service
```

### Make a task

```bash
forge script script/NFTDemo.s.sol:TriggerNFTDemo \
    --sig "run(string)" \
    "How can I be a great artist?" \
    --rpc-url "http://localhost:8545" \
    --broadcast --gas-limit 100000000
```

### Show the result

```bash
forge script script/NFTDemo.s.sol:ShowLastResultNFTDemo --rpc-url "http://localhost:8545"
```
