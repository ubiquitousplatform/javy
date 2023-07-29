# TODO list

## Done

1. get ubiq_fn rust function compiling
1. get a regular js app to call Ubiquitous.Functions.Invoke and see the js and rust output

## Doing

1. run this javy module from .NET WASMTIME

## TODO

1. update javy library to call .NET code and log from .NET
1. update to actually return a value from .NET and print it out from rust
1. work on returning rust layer to js layer back to userspace js
1. investigate whether the rust / js implementation is necessary or is adding overhead, or if we can directly register the .NET function somehow
1. switch to a dynamically linked javy build
1. set up CI to automatically build custom javy module
1. create cli template that uses the CI build to make a new js function
1. implement logging in .NET to SQLite DB
1. use Tremor to visualize logs / traces

## Future

1. add support for messagepack and json
2. Post to Javy about whether we can serialize JS or messagepack within Rust in Javy instead of on the JS side