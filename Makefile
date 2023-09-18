docker-compose-file=~/data/repo/shell/docker/docker-compose.yml


up:
# docker-compose up
	docker-compose -f $(docker-compose-file) up -d postgres

create:
# create database
	docker-compose -f $(docker-compose-file) exec postgres \
	psql -p 5432 -U postgres \
	-c "CREATE DATABASE caesar;"

psql:
# connect database
	docker-compose -f $(docker-compose-file) exec postgres \
	psql -p 5432 -U postgres -d caesar

