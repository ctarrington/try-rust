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

cargo watch -x 'run --release'

### warning

the parameters in queries are $1, $2, $3, etc for postgres, not ? like in sqlite!

# To Do
* clean up magic strings - make it clear that they are magic X
* timestamp should be provided by the client - why hard way, why not millis since epoch?
* id and object id, one is a row id, the other is a uuid for the the real world object that measurements are made of
  over time
* add a custom error type  
* convert string to uuid on the way into the database  
* add a client sensor that adds records - rust binary with some config
* add a api endpoint to read records based on time
* add a client reader that reads records based on time - rust binary with some config
* add a client reader that reads records based on time - web page
* take some benchmarks
* ------------------------------------------------
* how to store json ish (protobuf, json, *flatbuffers*) for the sparse and crazy parts of the model
* add filtering on sparse data and repeat benchmarks
* ------------------------------------------------
* image frame db


# curl commands

curl -X POST http://localhost:8000/api/measurement -H "Content-Type: application/json" -d '{"measured_at":"2024-06-24T19:23:33.001001","object_id":"123456789012345678901234567890123456","latitude":33.65,"longitude":23.99,"object_length":10.2}'
