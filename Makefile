docker-compose-file=~/data/repo/shell/docker/docker-compose.yml
database-uri=postgres://postgres:postgres@localhost:5432/caesar

up:
# docker-compose up
	docker-compose -f $(docker-compose-file) up -d postgres

psql:
# connect database
	docker-compose -f $(docker-compose-file) exec postgres \
	psql -p 5432 -U postgres -d caesar

create:
# create database
	docker-compose -f $(docker-compose-file) exec postgres \
	psql -p 5432 -U postgres \
	-c "CREATE DATABASE caesar;"

migrate:
# migrate sea-orm-cli
	DATABASE_URL=$(database-uri) \
	sea-orm-cli migrate refresh

entity:
# generate entity sea-orm-cli
	sea-orm-cli generate entity \
	-u $(database-uri) \
	-o src/entities
