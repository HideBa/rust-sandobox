build:
	docker-compose build

db:
	docker-compose up -d db

dev:
	sqlx db create
	sqlx migrate run
	cargo watch -x run

test:
	cargo test
