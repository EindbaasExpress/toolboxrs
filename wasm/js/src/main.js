import init, { greet } from '../pkg/wasm'; // Import the WebAssembly module

init().then(() => {
    greet("WebAssembly");
  });