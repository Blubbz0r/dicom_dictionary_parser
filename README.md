# dicom_dictionary_parser

Library that allows to parse the various elements defined in DICOM standard
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