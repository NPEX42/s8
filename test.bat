echo "== Default =="
cargo run
echo "== Libcore =="
cargo run --features libcore
echo "== LibAlloc =="
cargo run --features liballoc