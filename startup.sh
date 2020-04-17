# Build the rust code & generate the `.wasm` file.
cd rust && wasm-pack build --release

# Switch to the js directory, and start the app using the WASM module.
cd ../js && npm run start