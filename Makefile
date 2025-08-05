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


proto-verify: ## Verify that generated protos are up to date
	@echo "Verifying protobuf files are up to date..."
	@$(MAKE) proto-gen
	@if git diff --exit-code src/prost/; then \
		echo "✅ Protobuf files are up to date!"; \
	else \
		echo "❌ Protobuf files are out of date! Please run 'make proto-gen' and commit the changes."; \
		exit 1; \
	fi

proto-update-forward: ## Pull latest forward proto definitions from dymension repo
	@echo "Updating forward proto definitions from dymension repo..."
	@if [ ! -d "../d-dymension" ]; then \
		echo "Error: ../d-dymension directory not found. Please clone the dymension repo there."; \
		exit 1; \
	fi
	@echo "Copying forward proto definitions..."
	mkdir -p gen/dymensionxyz/dymension/forward
	cp -rf ../d-dymension/proto/dymensionxyz/dymension/forward/*.proto gen/dymensionxyz/dymension/forward/
	@echo "Forward proto definitions updated!"
	@echo "Now run 'make proto-gen' to regenerate the Rust bindings."