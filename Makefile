docker-compose-file=~/data/repo/shell/docker/docker-compose.yml

up:
	docker-compose -f $(docker-compose-file) up -d postgres
psql:
	docker-compose -f $(docker-compose-file) exec postgres psql -p 5432 -U postgres -d caesar
