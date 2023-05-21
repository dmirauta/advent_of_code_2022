Problem READMEs have been generated from saved html pages `https://adventofcode.com/2022/day/<N>` with `pandoc *.html -o README.md` and trimmed.

For use of `cargo flamegraph`, the following is required in `.cargo/config`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-Clink-arg=-fuse-ld=lld", "-Clink-arg=-Wl,--no-rosegment"]
```
