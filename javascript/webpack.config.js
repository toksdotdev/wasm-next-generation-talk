const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./src/bootstrap.js"
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "src")
    ]),
  ]
};
