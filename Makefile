.PHONY: help fmt clean-db bake kill-local run run-local build clean

help:
	@echo "Available targets:"
	@echo "  help - Show this help"
	@echo "  fmt - Format code"
	@echo "  clean-infra - new postgres container with diesel migrations applied"
	@echo "  bake - Bake docker images"
	@echo "  kill-local - Kill local server and client processes"
	@echo "  run-local - Run server and client locally"
	@echo "  run - Run server and client in docker"
	@echo "  clean - Clean logs and target directories"
	@echo "  reload - Reload server and client locally"

fmt:
	@cargo fmt

clean-infra:
	@docker compose down && \
	docker compose up -d postgres vault && \
	sleep 2 && \
	cd database && \
	set -a && source .env && set +a && \
	diesel migration run && \
	cd -

bake:
	@docker buildx bake

run: bake
	@docker compose up -d

build:
	@cargo build

kill-local:
	@echo "Killing existing server and client processes..."
	@pkill -f "target/debug/(server|client)" || true

run-local: kill-local clean-infra build
	@$(MAKE) reload

reload: kill-local build
	@TIMESTAMP=$$(date +%Y%m%d_%H%M%S) && \
	mkdir -p logs/$$TIMESTAMP && \
	echo "Starting server" && \
	(cd server && set -a && . ./.env && set +a && exec cargo run > "../logs/$$TIMESTAMP/server.log" 2>&1 &) && \
	echo "Starting client" && \
	(cd client && set -a && . ./.env && set +a && exec cargo run > "../logs/$$TIMESTAMP/client.log" 2>&1 &) && \
	echo "Logs will be available in logs/$$TIMESTAMP/"

clean:
	@echo "Deleting logs and target directories..." && rm -rf logs target
