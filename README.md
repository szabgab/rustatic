# Rustatic - Rust Static server

When locally developing a static web site it can be quite useful to be able to run a small web server and be able to see the results.

Rustatic provides this service.


## Installation

### General public

* On the right-hand side of the [GitHub repository](https://github.com/szabgab/rustatic) you can find [Releases](https://github.com/szabgab/rustatic/releases).
* Download the latest one.

Then run

```
rustatic
```

or with more parameters run:

```
rustatic --indexfile index.html --nice --host 127.0.0.1 --port 5000 --path /path/to/html
```

Every command line parameter has a default value

* `--path` defaults to the current directory
* `--host` defaults to 127.0.0.1
* `--port` defaults to 5000
* `--indexfile` defaults to nothing. If provided then accessing a directory will yield the contenct of this file. Usually people have `--indexfile index.html`.
* `--nice` Server .html files with the extension. If the user accessed `/path/page`  rustatic will return the `/path/page.html` file.


### Rust users

* If you have Rust installed you can install `rustatic` using the following command:

```
cargo install rustatic
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


cargo run -- --indexfile index.html --nice --host 127.0.0.1 --port 5000 --path /path/to/html
```

## Release and publish

* Update version number in Cargo.toml and in the comment below.
* git add .
* git commit -m "update version to v0.2.4"
* git tag -a v0.2.4 -m "publish version v0.2.4"
* git push --tags
* cargo publish

## TODO

* create executable that people can download so no need to clone from repo
* Handle TODO items in the code.
