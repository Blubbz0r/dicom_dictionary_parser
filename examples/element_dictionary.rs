// This example shows how to generate a Rust source file that acts as a
// dictionary providing functions for the various data elements and file meta
// elements.
//
// The resulting "dictionary.rs" file will look like this (imagine that "Tag"
// is a struct holding group and element as u16):
// ```
// // File Meta Elements
//
// pub fn file_meta_information_group_length() -> Tag {
//     Tag::new(0x0002, 0x0000)
// }
//
// // ... other file meta elements
//
// // Data Elements
//
// pub fn length_to_end() -> Tag {
//     Tag::new(0x0008, 0x0001)
// }
//
// // ... other data elements
// ```

extern crate dicom_dictionary_parser as dict_parser;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() -> Result<(), Box<Error>> {
    let parser = dict_parser::Parser::new()?;
    let file_meta_elements = parser.parse_file_meta_element_registry()?;
    let data_elements = parser.parse_data_element_registry()?;

    let file = File::create("dictionary.rs")?;
    let mut buf_writer = BufWriter::new(file);

    buf_writer.write_all(
        b"// This file was automatically generated with dcmrs_dictionary_generatory\n\n",
    )?;

    buf_writer.write_all(b"use Tag;\n\n")?;

    buf_writer.write_all(b"// File Meta Elements\n\n")?;
    write_functions_for_data_elements(&mut buf_writer, &file_meta_elements)?;

    buf_writer.write_all(b"// Data Elements\n\n")?;
    write_functions_for_data_elements(&mut buf_writer, &data_elements)?;

    Ok(())
}

fn write_functions_for_data_elements<W: Write>(
    writer: &mut W,
    data_elements: &Vec<dict_parser::DataElement>,
) -> Result<(), Box<Error>> {
    for data_element in data_elements {
        // skip elements with empty keyword (e.g. 0018,0061)
        if data_element.keyword.is_empty() {
            continue;
        }

        // we do not add elements whose tag defines a range (e.g. VariablePixelData -> "(7Fxx,0010)")
        if data_element.tag[1..5].contains("x") || data_element.tag[6..10].contains("x") {
            continue;
        }

        // convert keyword to snake case
        let keyword = data_element
            .keyword
            .replace("\u{200b}", "_")
            .replace("__", "_") // some keywords include multiple zero-width spaces...
            .to_lowercase();
        writer.write_all(format!("pub fn {}() -> Tag {{\n", keyword).as_bytes())?;

        // data_element.tag is in format "(0008,0001)"
        let group = format!("0x{}", &data_element.tag[1..5]);
        let element = format!("0x{}", &data_element.tag[6..10]);
        writer.write_all(format!("    Tag::new({}, {})\n", group, element).as_bytes())?;

        writer.write_all(b"}\n\n")?;
    }

    Ok(())
}
