// This example shows how to generate a plain text file listing all SOP classes
// defined in the "Registry of DICOM Unique Identifieres (UIDs)" chapter of
// the DICOM standard.
//
// The resulting "sop_classes.txt" file will look like this:
// ```
// VerificationSOPClass,
// MediaStorageDirectoryStorage,
// ...
// ```

extern crate dicom_dictionary_parser as dict_parser;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() -> Result<(), Box<Error>> {
    let parser = dict_parser::Parser::new()?;
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
