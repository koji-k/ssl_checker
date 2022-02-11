# Check SSL expiration date

```bash
ssl_checker 
```

# Build
Build this project with musl toolchain.  
Using following Docker image makes it super easily.  

[rust-musl-builder: Docker container for easily building static Rust binaries](https://github.com/emk/rust-musl-builder)

```bash
sudo docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release
```

binary will be created under `./target/x86_64-unknown-linux-musl/release/`

# TODO
- add test (how...?)
- update CLI
