.PHONY: help fmt clean-db

help:
	@echo "Available targets:"
	@echo "  fmt - Format code"
	@echo "  clean-db - Clean database"
	@echo "  help - Show this help"

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