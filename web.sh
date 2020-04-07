# build rust to wasm
echo "[INFO] building rust to wasm"
wasm-pack build --target  web

# start a web server
echo "[INFO] start a web server"
python -m SimpleHTTPServer