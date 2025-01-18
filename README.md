# labjack_ud_rs: Rust Bindings for the LabJack UD Library
This package is a Rust binding for the LabJack UD library, allowing you to connect to and operate LabJack's U series devices directly from Rust code. As with the UD library itself, this package is only compatible with Rust code deployed on Windows

Documentation for LabJack's UD can be found here: https://support.labjack.com/docs/ud-library-user-s-guide

This binding is built using the bindgen library; doccumentation for bindgen can be found here: https://rust-lang.github.io/rust-bindgen/

Disclaimer: I'm not a professional, so if anyone has any thoughts/comments/ideas on how things in this package can be done better, please share! Pull requests/emails about this package are always welcome! Thanks!

## User guide
This package is very barebones, and essentially only provides the most basic wrapper of the UD's C++ functions. A few notes:

- When connected, a LabJack device is refered to by a pointer to an ```i32``` called its handle. Connection functions should return a device's handle, and this handle is then used for all subsequent UD calls to interact with the device. An example function to connect to a U6 device is included in ```lib.rs``` in this package.
- All UD calls must be warpped in an ```unsafe{}``` block

## Example projects using this library
- https://github.com/Callum-Welsh/Mag3tometer
- More coming soon?
If you use this library for your project and are willing to publish your code, please submit a pull request or send me an email so we can put your code on the examples!
