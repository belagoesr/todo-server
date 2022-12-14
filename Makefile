run-local:
	cargo run --features "db-test"

run:
	docker-compose up --build

down:
	docker-compose down

db:
	docker run -i --rm --name auth-db -p 5432:5432  -e DB_HOST=postgres -e POSTGRES_USER=auth -e POSTGRES_PASSWORD=secret -d postgres
 
clear-db:
	docker ps -a | awk '{ print $1,$2 }' | grep postgres | awk '{print $1 }' | xargs -I {} docker stop {}

int: db
	sleep 2
	diesel setup --migration-dir src/migrations
	diesel migration run --migration-dir src/migrations
	cargo test --test lib --no-fail-fast --features "dbtest" -- --test-threads 3
	diesel migration redo

unit:
	cargo test --locked --no-fail-fast --lib -- --test-threads 3

test: unit int