# ttc

Count tokens in files with a sort of `wc -l`-flavoured usage.

```
$ git ls-files | xargs ttc
       2 .gitignore
    4902 Cargo.lock
      72 Cargo.toml
     201 LICENSE
     185 README.md
     371 src/main.rs
     284 ttc
    6017 total
$ ttc < README.md
185
```

There are two implementations here. `ttc` is a python implementation. You can
just put it on your path and run `pip3 install tiktoken` or `pip install tiktoken`;
whichever makes it work.

The other implementation is in rust. Run `cargo build --release` and put
`target/release/ttc` on your path, or see if there's something on the releases
page for your platform.
