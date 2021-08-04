function fetchAndInstantiate(url, importObject) {
  return WebAssembly.instantiateStreaming(fetch(url), importObject)
    .then(results => results.instance);
}
