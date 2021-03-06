rusroonga
=========

* Yet another rust binding for [groonga](https://github.com/groonga/groonga).
* API are not frozen yet.
* I will implement only APIs which I use.

The existing rust binding for groonga:

* [cosmo0920/ruroonga](https://github.com/cosmo0920/ruroonga)


## How to generate groonga.rs with rust-bindgen

Run following command on OSX

```
brew install groonga
git clone https://github.com/crabtw/rust-bindgen
cd rust-bindgen
cargo build
DYLD_LIBRARY_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib \
  ./target/debug/bindgen -l groonga -match groonga.h -o groonga.rs /usr/local/include/groonga/groonga.h
```

## How to run test

```
RUST_TEST_THREADS=1 cargo test
```

See http://doc.rust-lang.org/1.1.0/src/test/lib.rs.html#336 for `RUST_TEST_THREADS` environment variable.

## License

* rusroonga: [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [groonga](https://github.com/groonga/groonga): [LGPL 2.1](https://github.com/groonga/groonga/blob/master/COPYING)
