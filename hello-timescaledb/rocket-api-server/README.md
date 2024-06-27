# Sources

https://github.com/rwf2/Rocket/blob/v0.5.1/examples/databases/src/sqlx.rs

# Database Notes

## Install Timescaledb in Docker

https://docs.timescale.com/self-hosted/latest/install/installation-docker/

Used Docker Desktop on mac
Used podman on pop-os

## install sqlx-cli

cargo install sqlx-cli

migrations are used to create the database    
create a .env file with    
DATABASE_URL=postgres://postgres:password@localhost/rocket_api_database    
and your actual non stupid password    
Also add it to Rocket.toml

## Nuke the database and recreate it

sqlx database reset

## Run the api server

cargo watch -x 'run --release --bin api_server'

## Run the sensor sim

cargo run --release --bin sensor_sim

### warning

the parameters in queries are $1, $2, $3, etc for postgres, not ? like in sqlite!

# To Do

* clean up magic strings - make it clear that they are magic X
* timestamp should be provided by the client - use pretty formated string for now in the request X
* id and object id, one is a row id, the other is a uuid for the real world object that measurements are made of
  over time X
* convert model uuid to sqlx uuid on the way into the database X
* time stamp as chrono NaiveDateTime in model and then TIMESTAMP in postgres X
* add a custom error type with anyhow X
* add a sensor id uuid X

------------------------------------------------

* multiple binaries in one cargo project? api server, sensor client, reader client?
* add a client sensor that adds records - rust binary with some config
    * sensor location and range and number of world objects via parameters
    * deterministic object points
    * sensor measures objects in range and sends them to the server with fuzz added
* add a api endpoint to read records based on time
* add a client reader that reads records based on time - rust binary with some config
* add a client reader that reads records based on time - web page
* take some benchmarks

* ------------------------------------------------
* how to store json ish (protobuf, json, *flatbuffers*) for the sparse and crazy parts of the model
* add filtering on sparse data and repeat benchmarks

------------------------------------------------

* add an api to subscribe to records based on time via SSE
* change the client to use SSE
* take some benchmarks
* ------------------------------------------------
* image frame db

# curl commands

Note: get rid of the spaces!

curl -X POST http://localhost:8000/api/measurement -H "Content-Type: application/json"
-d '{"measured_at":"2024-06-24T19:23:33.001234","object_uuid":"a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8",
"sensor_uuid":"e1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8", "latitude":33.65,"longitude":23.99,"object_length":99.9}'
