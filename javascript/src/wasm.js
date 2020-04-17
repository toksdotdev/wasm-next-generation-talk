import * as wasm from "testing-wasm";

// Stringify Json.
const json = require('./data/citm_catalog.json');
const stringified = JSON.stringify(json);

// expects Json object to return from wasm.
const parsed = wasm.parse(stringified);
console.log('Rust parsing completed from JS.', parsed);
console.log('Type returned is:', typeof parsed);
