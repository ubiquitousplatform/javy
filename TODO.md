# Done

1. get ubiq_fn rust function compiling
# Doing

1. get a regular js app to call Ubiquitous.Functions.Invoke and see the js and rust output

# TODO

3. run this javy module from .NET WASMTIME
4. update javy library to call .NET code and log from .NET
5. update to actually return a value from .NET and print it out from rust
6. work on returning rust layer to js layer back to userspace js
7. investigate whether the rust / js implementation is necessary or is adding overhead, or if we can directly register the .NET function somehow
8. add support for both messagepack and json
9. switch to a dynamically linked javy build
10. set up CI to automatically build custom javy module
11. create cli template that uses the CI build to make a new js function
12. implement logging in .NET to SQLite DB
13. use Tremor to visualize logs / traces

