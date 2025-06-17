# Hyperlane Cosmos Rust

This package provides proto and gRPC definitions to interact with the [Hyperlane Cosmos Module](https://github.com/dymensionxyz/hyperlane-cosmos) and Dymension's [x/kas module](https://github.com/dymensionxyz/dymension).

This repository builds upon [cosmrs](https://github.com/cosmos/cosmos-rust), using their workflow for Hyperlane-specific proto builds.

## Features

- **Hyperlane Core**: Message dispatch and processing capabilities
- **Hyperlane Warp**: Token bridging functionality  
- **x/kas**: Dymension's Kaspa escrow module for cross-chain operations

## Building Proto files from source

There is a docker container that automatically builds all the files from source. To re-export the generated files adjust [lib.rs](./src/lib.rs). To start the docker container simply run:

    cd gen
    docker compose up
