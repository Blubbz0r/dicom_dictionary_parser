# dicom_dictionary_parser

[![Crates.io](https://img.shields.io/crates/v/dicom_dictionary_parser.svg)](https://crates.io/crates/dicom_dictionary_parser)
[![Documentation](https://docs.rs/dicom_dictionary_parser/badge.svg)](https://docs.rs/dicom_dictionary_parser)
[![Crates.io](https://img.shields.io/crates/l/dicom_dictionary_parser.svg)](https://crates.io/crates/dicom_dictionary_parser)

A Rust library that allows to parse the various elements defined in DICOM standard
part 6. It returns a simple data structure representing the definitions that
can be used to e.g. automatically generate a dictionary source file with all
elements currently defined by the DICOM standard. It is necessary to
automatically generate such files due to the sheer amount of elements defined
by the DICOM standard (thousands).

## Usage

Add this to your 'Cargo.toml':

```toml
[dependencies]
dicom_dictionary_parser = "0.1.0"
```

and this to your crate root:

```rust
extern crate dicom_dictionary_parser;
```