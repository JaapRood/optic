const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = function createWasmpackPlugin() {
  return new WasmPackPlugin({
    crateDirectory: __dirname,
    outDir: 'pkg',
    outName: 'index',
  });
};
