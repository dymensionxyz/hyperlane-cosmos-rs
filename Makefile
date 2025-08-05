.PHONY: proto-gen proto-clean proto-setup help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

proto-gen: ## Generate protobuf files using Docker
	@echo "Generating protobuf files..."
	cd gen && docker compose up --build --force-recreate
	@echo "Protobuf generation complete!"

proto-clean: ## Clean generated protobuf files
	@echo "Cleaning generated protobuf files..."
	rm -rf src/prost/*.rs
	cd gen && docker compose down --remove-orphans
	@echo "Clean complete!"

proto-setup: ## Setup IBC proto dependencies
	@echo "Setting up IBC proto dependencies..."
	cd gen && mkdir -p ibc/applications/transfer/v1
	cd gen && cp tx.proto ibc/applications/transfer/v1/
	cd gen && cp transfer.proto ibc/applications/transfer/v1/
	@echo "IBC proto setup complete!"

proto-verify: ## Verify that generated protos are up to date
	@echo "Verifying protobuf files are up to date..."
	@$(MAKE) proto-gen
	@if git diff --exit-code src/prost/; then \
		echo "✅ Protobuf files are up to date!"; \
	else \
		echo "❌ Protobuf files are out of date! Please run 'make proto-gen' and commit the changes."; \
		exit 1; \
	fi

proto-update-dymension: ## Pull latest proto definitions from dymension repo
	@echo "Updating proto definitions from dymension repo..."
	cd gen && rm -rf /tmp/dymension-proto-update
	cd gen && git clone https://github.com/dymensionxyz/dymension.git /tmp/dymension-proto-update
	cd gen && mkdir -p dymensionxyz/dymension
	cd gen && cp -rf /tmp/dymension-proto-update/proto/dymensionxyz/dymension/kas ./dymensionxyz/dymension/
	cd gen && cp -rf /tmp/dymension-proto-update/proto/dymensionxyz/dymension/forward ./dymensionxyz/dymension/
	cd gen && rm -rf /tmp/dymension-proto-update
	@echo "Proto definitions updated!"
	@echo "Now run 'make proto-gen' to regenerate the Rust bindings."