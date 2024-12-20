# freefare-sys

[![Crates.io](https://img.shields.io/crates/v/freefare-sys.svg?maxAge=2592000)](https://crates.io/crates/freefare-sys)

`freefare-sys` provides FFI bindings to [libfreefare](https://github.com/nfc-tools/libfreefare).

Following the `*-sys` package conventions, the `freefare-sys` package does not define higher-level abstractions over the native library.

## Requirements

You need to install `libfreefare` and `libnfc` before using this crate.

### MacOS + Homebrew

```bash
brew install libfreefare libnfc
```

### Ubuntu + APT

```bash
sudo apt install libfreefare-dev libnfc-dev
```

### Custom Paths

If the two libraries above are not installed in the standard Homebrew or APT locations, you can override the following environment variables:

* **LIBFREEFARE_PATH**: Path to libfreefare (e.g. **/path/to/libfreefare/lib**).
* **LIBNFC_PATH**: Path to libnfc (e.g. **/path/to/libnfc/lib**).

See `build.rs` for more details on how it works if needed.

## Usage

Add both `libc` and `freefare-sys` as dependencies in your `Cargo.toml`. 

```toml
[dependencies]
libc = "0.2.0"
freefare-sys = "0.2.1"
```

## Example

Note: this example requires both the `freefare_sys` and `nfc_sys` crates.

```rust
extern crate nfc_sys;
extern crate freefare_sys;

use std::ffi::CStr;
use std::ptr;

fn main() {
    unsafe {
        // Initialize NFC context
        let mut context: *mut nfc_sys::nfc_context = ptr::null_mut();
        nfc_sys::nfc_init(&mut context);

        if context.is_null() {
            eprintln!("Failed to initialize NFC context.");
            return;
        }
        println!("NFC context initialized.");

        // Open the NFC device
        let device = nfc_sys::nfc_open(context, ptr::null());
        if device.is_null() {
            eprintln!("Failed to open NFC device.");
            nfc_sys::nfc_exit(context);
            return;
        }
        println!("NFC device opened.");

        // Get the list of Freefare tags
        let tags = freefare_sys::freefare_get_tags(device);

        if tags.is_null() {
            eprintln!("No Freefare tags found.");
        } else {
            let mut tag_ptr = tags;

            // Iterate through the array of tag pointers
            while !(*tag_ptr).is_null() {
                let tag = *tag_ptr;

                // Get and print the tag UID
                let uid = freefare_sys::freefare_get_tag_uid(tag);
                if !uid.is_null() {
                    let uid_str = CStr::from_ptr(uid).to_string_lossy();
                    println!("Found tag with UID: {}", uid_str);

                     // Free the UID string
                    freefare_sys::freefare_free_tag(uid as *mut _);
                }

                // Get and print the tag type
                let tag_type = freefare_sys::freefare_get_tag_type(tag);
                match tag_type {
                    freefare_sys::Enum_freefare_tag_type::MIFARE_CLASSIC_1K => {
                        println!("Tag type: MIFARE Classic 1K");
                    }
                    freefare_sys::Enum_freefare_tag_type::MIFARE_CLASSIC_4K => {
                        println!("Tag type: MIFARE Classic 4K");
                    }
                    freefare_sys::Enum_freefare_tag_type::MIFARE_ULTRALIGHT => {
                        println!("Tag type: MIFARE Ultralight");
                    }
                    _ => println!("Unknown tag type"),
                }

                // Move to the next tag in the array
                tag_ptr = tag_ptr.add(1);
            }
        }

        // Clean up the list of tags
        freefare_sys::freefare_free_tags(tags);
        println!("Completed Freefare tag scan.");

        // Close NFC device and exit context
        nfc_sys::nfc_close(device);
        nfc_sys::nfc_exit(context);
    }
}
```

## Contributing

If you've found a bug or have an idea, feel free to open an Issue. If you've got a fix or feature ready, open a PR. Thanks!

## License

MIT
