# once
rustup target add wasm32-unknown-unknown

# build the wasm code
cd rustwasmhello
cargo build --target wasm32-unknown-unknown

# launch the server
cd ../server
python3 -m http.server 8080

