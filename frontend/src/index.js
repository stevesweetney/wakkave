import('./app.js')
  .catch(e => {
    return import('./wasm_error.js')
  });
