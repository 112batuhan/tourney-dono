# cargo install just

set dotenv-load

run-rust:
    cargo run --release

generate-site:
    cd frontend && npm run generate && cp -r .output/public ..

run-db: 
    docker compose -f docker-compose.yml up postgres -d

run-server:
    docker compose -f docker-compose.yml up tourney-dono


run-local: run-db generate-site run-rust

run-docker: run-db run-server
