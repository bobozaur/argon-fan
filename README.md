# Cross-compile

```
cargo install cargo-deb
cargo install cross
```

```
cross build --release --target aarch64-unknown-linux-gnu
cargo deb --target aarch64-unknown-linux-gnu --no-build --no-strip
```
