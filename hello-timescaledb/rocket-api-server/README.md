# Sources

* https://github.com/rwf2/Rocket/blob/v0.5.1/examples/databases/src/sqlx.rs
* https://www.postgresql.org/docs/current/queries-limit.html
* https://www.timescale.com/blog/13-tips-to-improve-postgresql-insert-performance/

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

cargo watch -x 'run --release --bin api-server'

## Build the client sim and sensor sim

cargo watch -x 'build --release --bin sensor-sim'
cargo watch -x 'build --release --bin client-sim'

## Run the sensor sim

./target/release/sensor-sim --future-count 4 --object-count 1000

## Run the client sim

./target/release/client-sim --window-minutes 1

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
* switch measurement id to be a generated uuid X
* add a database populated recorded time stamp X
* clean up the server return X
* make the sensor sim multi threaded X
* make the sensor sim configurable via cmd line parameters X

------------------------------------------------

* multiple binaries in one cargo project X
* add a client sensor that adds records - rust binary with some config
    * one sensor id X
    * deterministic object points on each iteration X
    * update the object id on each iteration X

------------------------------------------------

* add an api endpoint to read all records based on a time range X
* add a client reader that reads records based on time - rust binary
    * cmd line option for ago and window duration X
    * print out the latest measured time, uuid, lat and long of each object X
* api should only return the latest record for each object X

------------------------------------------------

* add an api endpoint to read path history of a single object within a time range X
* add a details query to the client reader X
    * print out the history for a single object X

------------------------------------------------

* add a diagnostics endpoint to the api
    * the number of records in the database X
    * the number of objects in the database X
    * size of the database X

------------------------------------------------

* clean up server
    * split each route into its own file X
    * better way to handle uuid conversions X
    * better way to handle time conversions X
* clean up clients X
* add some variation to the sensor sim
    * drop an object sometimes X

------------------------------------------------

* test error handling for clients X
* test error handling for server X

------------------------------------------------

* add instrumentation to responses
    * times X
    * display with elapsed X
* add pagination X
* switch ago option in client to seconds X
* more fields X
* search for text with like X
* make text search optional
* search for value ranges with between for lat/long
* make value ranges optional
* sort by not time

---------- Performance Ideas ------------------

* https://www.timescale.com/blog/best-practices-for-picking-postgresql-data-types
    * do include timezone??
* https://www.timescale.com/blog/13-tips-to-improve-postgresql-insert-performance/
    * bulk insert
* floor request times to the nearest second
* floor request times to the nearest 10 seconds
* floor request times to the nearest 20 seconds
* mess with isolation level
* mess with interval
* mess with materialized views
* add sort order by time or length
* add timing to more spots: path,... sends

------------------------------------------------

* take some benchmarks
* try a materialized view for the latest record for each object
* experiment with different shard intervals

* ------------------------------------------------
* how to store json ish (protobuf, json, *flatbuffers*) for the sparse and crazy parts of the model
* add filtering on sparse data and repeat benchmarks

------------------------------------------------

* add an api to subscribe to records based on time via SSE
* change the client to use SSE
* take some benchmarks
* ------------------------------------------------
* minimal error handling in sensor sim
* make the sensor sim wait the balance of 1 second at the end of each tick

* ------------------------------------------------
* image frame db

# curl commands

Note: get rid of the spaces!

curl -X POST http://localhost:8000/api/measurement -H "Content-Type: application/json"
-d '{"measured_at":"2024-06-24T19:23:33.001234","object_uuid":"a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8",
"sensor_uuid":"e1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8", "latitude":33.65,"longitude":23.99,"object_length":99.9}'

curl -X
GET "http://localhost:8000/api/find_measurements?start=2024-06-28T22:01:10&end=2024-06-29T22:01:15&page_size=10&page_index=0" |
jq . |
vim -

curl -X
GET "http://localhost:8000/api/get_path?start=2024-06-29T01:01:01&end=2024-06-29T23:59:59&object_uuid=fff458c2-424c-42f4-96f8-ed4ac2e124f1" |
jq .

curl -X GET "http://localhost:8000/api/get_diagnostics" | jq .

# sql commands

select * from chunks_detailed_size('measurements')"

# podman commands

podman stats    
podman container ls    
podman top timescaledb    
podman run -d --name timescaledb -p 5432:5432 -e POSTGRES_PASSWORD=password docker.io/timescale/timescaledb-ha:pg16    
