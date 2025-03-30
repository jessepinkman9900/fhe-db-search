.PHONY: help fmt clean-db bake

help:
	@echo "Available targets:"
	@echo "  help - Show this help"
	@echo "  fmt - Format code"
	@echo "  clean-db - Clean database"
	@echo "  bake - Bake docker images"

fmt:
	cargo fmt

clean-db:
	docker compose down && \
	docker compose up -d && \
	sleep 2 && \
	cd db && \
	set -a && source .env && set +a && \
	diesel migration run && \
	cd -

bake:
	docker buildx bake
