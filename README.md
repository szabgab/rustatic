# Rustatic - Rust Static server

When locally developing a static web site it can quite useful to be able to run a small web server and be able to see the results.

Rustatic provides this service.

## How to use it:

* Install Rust

```
git clone https://github.com/szabgab/rustatic
cd rustatic
cargo run -- --help


cargo run -- --path /some/path/where/you/have/your/static/site --host 127.0.0.1 --port 5000
```

* `path` defaults to the current directory
* `host` defaults to 127.0.0.1
* `port` defaults to 5000
* `indexfile` if provided then accessing a directory will yield the contenct of this file. Usually people have `index.html`.

## TODO

* create executable that people can download so no need to clone from repo
* Handle TODO items in the code.
