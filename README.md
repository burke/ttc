# ttc

Count tokens in files with a sort of `wc -l`-flavoured usage.

```
$ git ls-files | xargs ttc
     201 LICENSE
      95 README.md
     284 ttc
     580 total
$ ttc < README.md
95
```

Put it on your path and run `pip3 install tiktoken` or `pip install tiktoken`;
whichever makes it work.
