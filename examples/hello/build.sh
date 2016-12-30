cargo build --target asmjs-unknown-emscripten -v
cp target/asmjs-unknown-emscripten/debug/hello.js static/webplatform.js
python -m SimpleHTTPServer 8080
#open http://localhost:8080/static/index.html
