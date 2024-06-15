include .env
ENV=DISCORD_TOKEN=${DISCORD_TOKEN} DATABASE_URL=${DATABASE_URL}
run_bot:
	${ENV} cargo run --bin bot
test:
	cargo test

# DATABASE
.PHONY: db_clean db_migrate db_create db_prepare
DB_NAME = capitoltrades_db

db: db_create db_migrate

db_create: db_clean
	sqlx database create

db_migrate:
	sqlx migrate run

db_create_migration:
	sqlx migrate add $(name)

db_clean:
	rm -f ${DB_NAME}*

db_prepare:
	cargo sqlx prepare --workspace -- --all-features --all-targets
