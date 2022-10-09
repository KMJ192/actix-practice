import wasm from "./wasm-module/pkg/wasm_module.js";

async function run() {
  (await wasm()).app();
}

run();
