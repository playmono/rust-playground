SERVICE_NAME ?= rust_playground

build:
	@test -s .env || cp .env.dist .env
	docker-compose -p $(SERVICE_NAME) build --pull

stop:
	docker-compose -p $(SERVICE_NAME) down

run:
	docker-compose run $(SERVICE_NAME)