Remove-Item javy-example.wasm
echo "compiling javy-example.js into javy-example.wasm"
./target/release/javy compile javy-example.js -o javy-example.wasm