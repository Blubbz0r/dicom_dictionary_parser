extern crate dicom_dictionary_parser as dict_parser;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() -> Result<(), Box<Error>> {
    let parser = dict_parser::Parser::new()?;

    write_dictionary(&parser)?;
    write_sop_classes(&parser)?;

    Ok(())
}

fn write_dictionary(parser: &dict_parser::Parser) -> Result<(), Box<Error>> {
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

fn write_sop_classes(parser: &dict_parser::Parser) -> Result<(), Box<Error>> {
    let uids = parser.parse_unique_identifier_registry()?;

    let file = File::create("sop_classes.txt")?;
    let mut buf_writer = BufWriter::new(file);

    for uid in uids {
        if uid.kind == dict_parser::Kind::SopClass {
            let sop_class = uid
                .normalized_name
                .replace(" ", "")
                .replace("-", "")
                .replace("(Retired)", "")
                .replace("(", "")
                .replace(")", "")
                .replace("/", "");

            if !sop_class.is_empty() {
                buf_writer.write_all(format!("{},\n", sop_class).as_bytes())?;
            }
        }
    }

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
