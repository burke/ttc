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
Run `cargo build --release` then copy `target/release/ttc` to your path.

You can select between encodings (`cl100k_base`, `p50k_base`, `p50k_edit`,
`r50k_base`) with the `--encoding` flag. The default is `cl100k_base`.
