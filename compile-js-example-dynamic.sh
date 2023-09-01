rm javy-example.wasm
rm ubiquitous_quickjs_v1.wasm
echo "compiling javy-example.js into javy-example.wasm"
./target/release/javy compile javy-example.js -d -o javy-example.wasm
echo "emitting provider into ubiquitous_quickjs_v1.wasm"
./target/release/javy emit-provider -o ubiquitous_quickjs_v1.wasm