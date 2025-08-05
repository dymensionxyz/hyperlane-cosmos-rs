# Protobuf Update Guide

This guide explains how to update the Rust protobuf implementations when the proto definitions change in the dymension repository.

## Quick Start

When proto definitions are updated in the dymension repository:

1. **Update proto definitions from dymension repo:**
   ```bash
   make proto-update-dymension
   ```

2. **Regenerate Rust bindings:**
   ```bash
   make proto-gen
   ```

3. **Verify the update:**
   ```bash
   make proto-verify
   # OR
   ./scripts/verify-protos.sh
   ```

## Manual Process

If you need to manually update specific proto files:

1. **Copy proto files to the gen directory:**
   ```bash
   cd gen
   # Clone the dymension repo temporarily
   git clone https://github.com/dymensionxyz/dymension.git /tmp/dymension-update
   
   # Copy the proto files you need
   cp -rf /tmp/dymension-update/proto/dymensionxyz/dymension/forward ./dymensionxyz/dymension/
   cp -rf /tmp/dymension-update/proto/dymensionxyz/dymension/kas ./dymensionxyz/dymension/
   
   # Clean up
   rm -rf /tmp/dymension-update
   ```

2. **Ensure dependencies are set up:**
   ```bash
   make proto-setup  # Sets up IBC proto dependencies
   ```

3. **Generate the Rust code:**
   ```bash
   make proto-gen
   ```

## How It Works

The protobuf generation process uses Docker to ensure consistency:

1. The `gen/Dockerfile` clones both `hyperlane-cosmos` and `dymension` repositories
2. Proto files are copied from both repos into the build context
3. `buf generate` is run to create the Rust bindings using prost
4. Generated files are output to `src/prost/`

## Troubleshooting

### Missing Dependencies

If you get errors about missing proto imports (e.g., IBC protos), ensure the IBC proto files are in place:

```bash
cd gen
mkdir -p ibc/applications/transfer/v1
# Copy the IBC proto files if not present
```

### Verification Failures

If `make proto-verify` fails, it means the generated code doesn't match expectations. Run:
```bash
make proto-gen
```

Then commit the updated files.

## Available Make Commands

- `make proto-gen` - Generate protobuf files
- `make proto-clean` - Clean generated files and Docker containers
- `make proto-setup` - Setup IBC proto dependencies
- `make proto-verify` - Verify protos are up to date
- `make proto-update-dymension` - Pull latest protos from dymension repo
- `make help` - Show all available commands

## Docker Requirements

The protobuf generation requires Docker to be installed and running. The process uses:
- `bufbuild/buf:latest` for protobuf compilation
- `node:20-alpine` as the base runtime environment