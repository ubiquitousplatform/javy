// TODO: we need to force-wrap an Init() method on the JS to control the Ubiquitous setup so that can be shifted out of the compiler into the lib,
//  and also need to capture any JS exceptions and call invoke_json(action:"js_exception")

// Read input from stdin
// const input = readInput();
// Call the function with the input
// const result = foo(input);
// Write the result to stdout
foo();
//writeOutput(result);

// The main function.
function foo(input) {
  var data = JSON.stringify({
    action: "log",
    payload: {
      message: "This is the input string from the client JS!",
      timestamp: new Date(),
    },
  });
  // console.log("About to call invoke_json!");
  // console.log(Ubiquitous);
  // console.log(Ubiquitous.Functions);
  // console.log(Ubiquitous.Functions.invoke_json);
  Ubiquitous.Functions.invoke_json(data);
  // return { foo: input.n + 1, newBar: input.bar + "!" };
  return { foo: 1, newBar: "!" };
}

// Read input from stdin
function readInput() {
  const chunkSize = 1024;
  const inputChunks = [];
  let totalBytes = 0;

  // Read all the available bytes
  while (1) {
    const buffer = new Uint8Array(chunkSize);
    // Stdin file descriptor
    const fd = 0;
    const bytesRead = Javy.IO.readSync(fd, buffer);

    totalBytes += bytesRead;
    if (bytesRead === 0) {
      break;
    }
    inputChunks.push(buffer.subarray(0, bytesRead));
  }

  // Assemble input into a single Uint8Array
  const { finalBuffer } = inputChunks.reduce(
    (context, chunk) => {
      context.finalBuffer.set(chunk, context.bufferOffset);
      context.bufferOffset += chunk.length;
      return context;
    },
    { bufferOffset: 0, finalBuffer: new Uint8Array(totalBytes) }
  );

  return JSON.parse(new TextDecoder().decode(finalBuffer));
}

// Write output to stdout
function writeOutput(output) {
  const encodedOutput = new TextEncoder().encode(JSON.stringify(output));
  const buffer = new Uint8Array(encodedOutput);
  // Stdout file descriptor
  const fd = 1;
  Javy.IO.writeSync(fd, buffer);
}
