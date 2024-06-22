### run

cargo watch -x "run --release"

### listen for server sent events

curl -N http://localhost:8000/events

### send a message to the server

curl -X POST http://localhost:8000/forms/message -H "Content-Type: application/x-www-form-urlencoded" \
-d "from=fred&content=hello"

curl -X POST http://localhost:8000/data/message -H "Content-Type: application/json" \
-d '{"from":"ted","content":"no way"}'