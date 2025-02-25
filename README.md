# [WAVS](https://docs.wavs.xyz) Demo Services

<!-- ![Rust](https://github.com/gakonst/foundry-rust-template/workflows/Rust/badge.svg)
![Solidity](https://github.com/gakonst/foundry-rust-template/workflows/Solidity/badge.svg) -->

**Demos of WAVS Rust and Solidity services**

Comprehensive demos for developing WAVS (WebAssembly AVS) applications using Rust and Solidity. This repo provides a pre-configured development environment with integrated testing frameworks for both Rust and Solidity components.

## Demos

- [NFT + AI Demo](./demos/NFT_AI_DEMO.md) -
  An NFT is minted by an AVS with AI-generated metadata.
- [Prediction Market Oracle Demo](./demos/PREDICTION_MARKET_DEMO.md) -
  A prediction market is resolved by an AVS oracle.
- [Safe AI Module Demo](./demos/SAFE_AI_MODULE_DEMO.md) -
  Transactions in a Safe are created by an AVS using an AI agent.
- [Safe Guard Demo](./demos/SAFE_GUARD_DEMO.md) -
  An AVS acts as a Safe Guard to validate and execute a Safe transaction.

## System Requirements

<details>
<summary>Core (Docker, Compose, Make, JQ, Node v21+)</summary>

### Docker

- **MacOS**: `brew install --cask docker`
- **Ubuntu**: `sudo apt -y install docker.io`
- [Docker Documentation](https://docs.docker.com/get-started/get-docker/)

### Docker Compose

- **MacOS**: Already installed with Docker installer
- **Linux**: `sudo apt-get install docker-compose-v2`
- [Compose Documentation](https://docs.docker.com/compose/)

### Make

- **MacOS**: `brew install make`
- **Linux**: `sudo apt -y install make`
- [Make Documentation](https://www.gnu.org/software/make/manual/make.html)

### JQ

- **MacOS**: `brew install jq`
- **Ubuntu**: `sudo apt -y install jq`
- [JQ Documentation](https://jqlang.org/download/)

### Node.js

- **Required Version**: v21+
- [Installation via NVM](https://github.com/nvm-sh/nvm?tab=readme-ov-file#installing-and-updating)
</details>

<details>

<summary>Rust v1.84+</summary>

### Rust Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup toolchain install stable
rustup target add wasm32-wasip2
```

### Upgrade Rust

```bash
# Remove old targets if present
rustup target remove wasm32-wasi || true
rustup target remove wasm32-wasip1 || true

# Update and add required target
rustup update stable
rustup target add wasm32-wasip2
```

</details>

<details>
<summary>Cargo Components</summary>

### Install Cargo Components

```bash
# Install required cargo components
# https://github.com/bytecodealliance/cargo-component#installation
cargo install cargo-component warg-cli wkg --locked

# Configure default registry
wkg config --default-registry wa.dev
```

</details>

<details>
<summary>Foundry</summary>

### Install Foundry

https://book.getfoundry.sh/getting-started/installation

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

</details>
