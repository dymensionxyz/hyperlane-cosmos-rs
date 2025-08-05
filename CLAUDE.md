# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust library crate that provides protobuf definitions and gRPC client code for interacting with:
- Hyperlane Cosmos Module
- Dymension's x/kas module (Kaspa escrow functionality)
- IBC transfer protocols

## Common Development Commands

### Protobuf Generation
```bash
make proto-gen                   # Generate protobuf files using Docker
make proto-clean                 # Clean generated files and containers
make proto-setup                 # Setup IBC proto dependencies
make proto-verify                # Verify protos are up to date
make proto-update-dymension      # Pull latest protos from dymension repo
./scripts/verify-protos.sh       # Verify specific proto field requirements
```

### Rust Development
```bash
cargo check                      # Verify compilation
cargo build                      # Build the library
cargo test                       # Run tests
cargo publish                    # Publish to crates.io
```

## Architecture Overview

### Module Structure
The library is organized into three main namespace modules:

1. **hyperlane**: Core Hyperlane protocol definitions
   - `core`: Message dispatch and interchain security modules
   - `warp`: Token bridging protocols

2. **dymensionxyz**: Dymension-specific extensions
   - `dymension::kas`: Kaspa escrow functionality
   - `dymension::forward`: Message forwarding
   - `hyperlane::kaspa`: Kaspa integration for Hyperlane

3. **ibc**: Inter-Blockchain Communication protocol support
   - `applications::transfer`: IBC transfer protocols
   - `core::client`: IBC client protocols

### Protobuf Workflow

1. **Proto sources**: Cloned from `hyperlane-cosmos` (main-dym branch) and `dymension` repos
2. **Generation**: Uses Docker with `bufbuild/buf:latest` and prost/tonic plugins
3. **Output**: Generated files go to `src/prost/` (tracked in git)
4. **Verification**: Scripts ensure critical fields like `message_id` remain as bytes

### Key Dependencies
- **prost** v0.13 - Protocol buffer implementation
- **tonic** v0.12 - gRPC framework
- **cosmrs** v0.21.0 - Cosmos SDK Rust bindings
- **tendermint-proto** v0.40.0 - Tendermint protocol definitions

## Important Notes

- Rust toolchain version: 1.87.0 (specified in `rust-toolchain`)
- Generated proto files in `src/prost/` are committed to git
- The `verify-protos.sh` script checks that `message_id` fields remain as `bytes` type
- When updating protos, always run `make proto-verify` before committing
- The project is a library crate - no binary targets
- Proto generation uses Docker, so Docker must be running for `make proto-gen`