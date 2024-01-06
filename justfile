default:
	@just --choose

api: 
	cd api && cargo watch -x run

web:
	cd web && pnpm run dev

db_gen:
   cd api && cargo sqlx prepare

all:
	docker-compose up --build & just api & just web