# freefare-sys

[![Crates.io](https://img.shields.io/crates/v/freefare-sys.svg?maxAge=2592000)](https://crates.io/crates/freefare-sys)
[![Docs.rs](https://docs.rs/freefare-sys/badge.svg)](https://docs.rs/freefare-sys)

Raw FFI bindings to the public [`libfreefare`](https://github.com/nfc-tools/libfreefare) C API.

This crate intentionally stays low level. It exposes the upstream ABI for use by higher-level Rust wrappers and NFC tooling, and does not add safe abstractions over tag access, authentication, or memory management.

## What changed in `1.0.0`

Version `1.0.0` is a breaking cleanup release:

- The binding surface now tracks the public `freefare.h` API instead of exposing generator artifacts and non-header symbols.
- Opaque native types such as `MifareTag`, `MifareDESFireKey`, `Mad`, and `MifareDESFireAID` are no longer represented as fake Rust structs with internals.
- Old `Struct_*` and `Enum_*` bindgen names were replaced with stable Rust-facing FFI names.
- Unsound `Default` implementations based on `mem::zeroed()` were removed.
- Modern Rust FFI types are used throughout (`c_char`, `c_int`, `c_void`, `u8`, `u16`, `u32`, `usize`, `isize`).
- The crate now ships smoke tests for layout assumptions and native linking.

## Native dependencies

You need both `libfreefare` and `libnfc` installed on the system.

### macOS with Homebrew

```bash
brew install libfreefare libnfc
```

### Debian or Ubuntu

```bash
sudo apt install libfreefare-dev libnfc-dev
```

## Linking

`build.rs` links against `freefare`. The `nfc-sys` dependency links `libnfc`.

By default it checks common Homebrew and Linux library directories for `libfreefare`. If your `libfreefare` installation lives elsewhere, set either the preferred `LIBFREEFARE_LIB_DIR` variable or the legacy `LIBFREEFARE_PATH` variable:

```bash
export LIBFREEFARE_LIB_DIR=/custom/prefix/lib
```

## Usage

```toml
[dependencies]
freefare-sys = "1"
libc = "0.2"
nfc-sys = "1"
```

## Example

```rust,no_run
use std::ffi::CStr;
use std::ptr;

fn main() {
    unsafe {
        let mut context: *mut nfc_sys::nfc_context = ptr::null_mut();
        nfc_sys::nfc_init(&mut context);
        assert!(!context.is_null());

        let device = nfc_sys::nfc_open(context, ptr::null());
        if device.is_null() {
            nfc_sys::nfc_exit(context);
            return;
        }

        let tags = freefare_sys::freefare_get_tags(device);
        if !tags.is_null() && !(*tags).is_null() {
            let tag = *tags;
            let uid = freefare_sys::freefare_get_tag_uid(tag);

            if !uid.is_null() {
                println!("tag uid: {}", CStr::from_ptr(uid).to_string_lossy());
                libc::free(uid.cast());
            }

            freefare_sys::freefare_free_tags(tags);
        }

        nfc_sys::nfc_close(device);
        nfc_sys::nfc_exit(context);
    }
}
```

## Safety

This crate exposes raw C bindings. Callers are responsible for:

- Upholding all pointer validity and lifetime requirements from `libfreefare`.
- Freeing memory returned by the native library with the correct native deallocator.
- Ensuring linked `libfreefare` and `libnfc` versions are ABI-compatible with the declarations in this crate.

## Versioning

The Rust crate version follows the Rust FFI surface, not the upstream `libfreefare` release number. Breaking binding corrections or symbol removals may require a new major version even when the C library version is unchanged.

## License

MIT
