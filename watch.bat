watchexec -r -s SIGKILL -w "hobo" -w "client" -w "server" -w "shared" "wasm-pack build client --dev --target web --out-dir ../public/wasm && cargo run --package server"
