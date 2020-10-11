# Learn Rust with Benford's law

This is *Learn Rust with Benford's law - A practical introduction to the Rust programming language*.

You can find all the information you need to follow this project in
[this blog post](https://gliderkite.github.io/posts/learn-rust-with-benford).

This project is organized in workspaces, and you can run any of the chapters that
you find in the blog post with:

```console
$ cargo run --package chapter01
$ cargo run --package chapter02
$ RUST_LOG=debug cargo run --package chapter03
.
.
.
$ RUST_LOG=info cargo run --package chapter07 -- datasets/census.csv
```

Have fun learning Rust! :crab:
