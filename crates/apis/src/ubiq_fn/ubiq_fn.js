(function () {
  // Store a reference to the private(ish) rust functions in the global namespace so we can call it from our closure
  const __ubiquitous_functions_invoke = globalThis.__ubiquitous_functions_invoke;

  // Define a global reference that will be reachable from any JS code that runs inside a runtime that includes this API
  // This is essentially how we adapt the JS and provide JS-side processing and error handling before invoking the rust code.
  globalThis.Ubiquitous.Functions = {
    invoke(fd, data) {
      console.log("Made it to the JS wrapper!")
      if (!(data instanceof Uint8Array)) {
        throw TypeError("Data needs to be a Uint8Array");
      }
      return __ubiquitous_functions_invoke(
        fd,
        data.buffer,
        data.byteOffset,
        data.byteLength
      );
    },
  };

  // Remove the temporary rust functions from the global namespace since the JS wrapper should be called now
  // and there's no need to keep around the private(ish) reference
  Reflect.deleteProperty(globalThis, "__ubiquitous_functions_invoke");
})();
