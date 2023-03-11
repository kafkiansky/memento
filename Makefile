build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

memcached:
	docker-compose logs -f memcached