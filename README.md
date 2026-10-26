![build](https://github.com/uhppoted/uhppoted-lib-rs/workflows/build/badge.svg)

# uhppoted-lib-rs

Rust crate for interfacing to the UHPPOTE access controllers.


## Release Notes

_In Development_


## Installation


## Development

Requirements:
- rustc 1.99.1+
- cargo 1.99.1+
- clippy 0.0.302+


### Building from source

Assuming you have `rustc`, `cargo` and `make` installed:

```
git clone https://codeberg.org/uhppoted/uhppoted-lib-rs.git
cd uhppoted-lib-rs
make build-all
```

If you prefer building manually:
```
git clone https://codeberg.org/uhppoted/uhppoted-lib-rs.git
cd uhppoted-lib-rs
cd uhppoted && cargo build
cd ..
cd examples/cli && cargo build
```

## API


## License

MIT License

Copyright (c) 2026 uhppoted

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

[MIT](https://codeberg.org/uhppoted/uhpoted-lib-rs/blob/master/LICENSE). 

