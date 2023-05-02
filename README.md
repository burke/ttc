# ttc

Count tokens in files with a sort of `wc -l`-flavoured usage.

```
$ git ls-files | xargs ttc
      90 README.md
     284 ttc
     374 total
$ ttc < README.md
90
```

Put it on your path and run `pip3 install tiktoken` or `pip install tiktoken`;
whichever makes it work.
