//! Library providing acess to the elements defined in the various tables of
//! DICOM part 6. Currently, access to the following tables is provided:
//! * "Registry of DICOM Data Elements"
//! * "Registry of DICOM File Meta Elements"
//! * "Registry of DICOM Directory Structuring Elements"
//! * "Registry of DICOM Unique Identifiers (UIDs)"
//!
//! # Example - Writing data elements to file
//!
//! ```rust,no_run
//! extern crate dicom_dictionary_parser as dict_parser;
//!
//! use std::fs::File;
//! use std::io::BufWriter;
//! use std::io::Write;
//!
//! fn main() -> Result<(), Box<::std::error::Error>> {
//!     let parser = dict_parser::Parser::new()?;
//!     let data_elements = parser.parse_data_element_registry()?;
//!     let file = File::create("dictionary.rs")?;
//!     let mut buf_writer = BufWriter::new(file);
//!     for data_element in data_elements {
//!         let upper_case_keyword = data_element
//!             .keyword
//!             .replace("\u{200b}", "")
//!             .replace("__", "_")
//!             .to_uppercase();
//!
//!         buf_writer.write_all(
//!             format!(
//!                 "const {}: Tag = Tag(0x{}, 0x{});\n",
//!                 upper_case_keyword,
//!                 &data_element.tag[1..5],
//!                 &data_element.tag[6..10])
//!             .as_bytes())?;
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod data_element;
pub mod parser;
pub mod uid;

pub use data_element::DataElement;
pub use parser::Parser;
pub use uid::{Kind, UID};

extern crate reqwest;
extern crate xmltree;
