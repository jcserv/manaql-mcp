include .env
export

.PHONY: build test run run-watch dev-db dev-db-down migrate test-db test-db-down

build:
	cargo build

test:
	cargo test

run:
	cargo run

run-watch:
	cargo watch -q -c -w src/ -x run

dev-db:
	docker compose -p mtg-mcp -f docker-compose.yml up --detach

dev-db-down:
	docker compose -p mtg-mcp -f docker-compose.yml down -v

migrate:
	sqlx migrate run

test-db:
	docker compose -p mtg-mcp-test -f docker-compose.test.yml up --detach

test-db-down:
	docker compose -p mtg-mcp-test -f docker-compose.test.yml down -v