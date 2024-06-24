# Sources

https://github.com/svenstaro/rust-timescale-sqlx-rocket

# Database Notes

## Install Timescaledb in Docker

https://docs.timescale.com/self-hosted/latest/install/installation-docker/

Used Docker Desktop

## install sqlx-cli

cargo install sqlx-cli

## Create a database

create database rocket_api_database
create a .env file with    
DATABASE_URL=postgres://postgres:password@localhost/rocket_api_database    
and your actual non stupid password
Also add it to Rocket.toml

## Nuke the database

sqlx database reset

## Run the api server

cargo watch -x 'run --release'

### warning

the parameters in queries are $1, $2, $3, etc for postgres, not ? like in sqlite!

# To Do

* id and object id, one is a row id, the other is a uuid for the the real world object that measurements are made of
  over time
* timestamp should be provided by the client

# curl commands

curl -X POST http://localhost:8000/api/measurement -H "Content-Type: application/json" \\n-d '{"latitude":12.34,"
longitude":23.45,"object_length":10.1}'
