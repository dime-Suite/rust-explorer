## run server and tests in watch mode
$ cargo install cargo-watch

cargo watch -q -c -w examples/ -x "run --example main"

cargo watch -q -c -w src/ -x "cargo run"