Remove-Item javy-example.wasm
Remove-Item javy_quickjs_provider_v1.wasm
echo "compiling javy-example.js into javy-example.wasm"
./target/release/javy compile javy-example.js -d -o javy-example.wasm
echo "emitting provider into javy_quickjs_provider_v1.wasm"
./target/release/javy emit-provider -o javy_quickjs_provider_v1.wasm