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

[MIT](https://codeberg.org/uhppoted/uhpoted-lib-rs/blob/master/LICENSE). 
