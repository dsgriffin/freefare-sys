# freefare-sys

[![Crates.io](https://img.shields.io/crates/v/freefare-sys.svg?maxAge=2592000)](https://crates.io/crates/freefare-sys)

`freefare-sys` provides FFI bindings to [libfreefare](https://github.com/nfc-tools/libfreefare).

Following the `*-sys` package conventions, the `freefare-sys` package does not define higher-level abstractions over the native library.

## Requirements

Make sure you've got `libnfc` installed (e.g. [Debian/Ubuntu](http://nfc-tools.org/index.php?title=Libnfc#Debian_.2F_Ubuntu), `brew install libnfc` using Homebrew on Mac OS X, or on [other systems](http://nfc-tools.org/index.php?title=Libnfc#Installation)).

Also [`libfreefare`](http://nfc-tools.org/index.php?title=Libfreefare) of course (`brew install libfreefare` using Homebrew on Mac OS X).

### Cargo.toml
```toml
[dependencies]
libc = "0.2.0"
freefare-sys = "0.2.0"
```
## Contributing

I'm brand new to Rust so any help or constructive information would be really appreciated. Thanks in advance!

## License

MIT
