## installation
nvm use    
cargo install wasm-pack    

## build rust
cd wasm    
find . -name "*.rs" | entr -r -c wasm-pack build    

## run react
cd react-client    
nvm use
npm install
npm start    
