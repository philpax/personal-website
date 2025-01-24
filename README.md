# paxsite

Use

    cargo watch -x clippy -x 'test --workspace' -x 'run -F development' --poll

to develop. (The `--poll` flag is needed as change detection sometimes detects changes to `contents` when they're read.)

Also, consider running with `RUSTFLAGS="-Clink-arg=-fuse-ld=lld"` if on Linux x86-64. (The default linker is not very
fast; a future version of Rust should default to `rust-lld` and fix this.)