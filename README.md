# Rustatic - Rust Static server

When locally developing a static web site it can quite useful to be able to run a small web server and be able to see the results.

Rustatic provides this service.


## Installation

### General public

* We will prepare binary executables for you.

Every command line parameter has a default value

* `--path` defaults to the current directory
* `--host` defaults to 127.0.0.1
* `--port` defaults to 5000
* `--indexfile` defaults to nothin. if provided then accessing a directory will yield the contenct of this file. Usually people have `--indexfile index.html`.
* `--nice` Server .html files with the extension. If the user accessed `/path/page`  return the `/path/page.html` file.


### Rust users

* If you have Rust installed you can install `rustatic` using the following command:

```
cargo install ruststatic
```

* Then you can run

```
rustatic --help
rustatic --version
rustatic --path /path/to/html --host 127.0.0.1 --port 5000
```

### Developers

If you would like to help with the development of `rustatic` you can clone the repo and run the program without further installation:

```
git clone https://github.com/szabgab/rustatic
cd rustatic
```

Then you can run the command like this:

```
cargo run -- --help
cargo run -- --version


cargo run -- --path /path/to/html --host 127.0.0.1 --port 5000
cargo run -- --indexfile index.html --nice --host 127.0.0.1 --port 5000 --path /path/to/html
```

## Release and publish

* Update version number in Cargo.toml
* git commit
* git tag
* cargo publish

## TODO

* create executable that people can download so no need to clone from repo
* Handle TODO items in the code.
