import('./app.js')
  .catch(e => import('./wasm_error.js'));
