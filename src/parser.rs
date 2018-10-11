use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use DataElement;

use reqwest;
use xmltree;

/// A parser for the data elements defined in various tables in the DICOM
/// standard (part 6 "Data Dictionary").
pub struct Parser {
    /// Holds the contents of the DICOM standard part 6 xml file once read.
    part6_content: String,
}

impl Parser {
    /// Creates a new `Parser` instance with a downloaded version of the
    /// current part 6 of the DICOM standard.
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// * Downloading part6.xml fails
    /// * Reading the downloaded part6.xml fails
    pub fn new() -> Result<Self, Box<Error>> {
        Ok(Parser {
            part6_content: Self::download_part_6()?,
        })
    }

    /// Creates a new `Parser` instance using the part6.xml given as `file_path`.
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// * Opening the file at `file_path` fails
    /// * Reading the file at `file_path` fails
    pub fn with_part6_file(file_path: &Path) -> Result<Parser, ::std::io::Error> {
        let mut file = File::open(file_path)?;
        Ok(Parser {
            part6_content: Self::read_content(&mut file)?,
        })
    }

    /// Returns all data elements defined in the "Registry of DICOM Data
    /// Elements" table of the DICOM standard.
    ///
    /// Note that **all** data elemnts from the dictionary are returned,
    /// including elements:
    ///
    /// * without name/keyword (e.g. "(0018,0061)""
    /// * with tags defining ranges (e.g. "EscapeTriplet" -> "(1000,xxx0)")
    /// * without VR (e.g. "Item" -> "(FFFE,E000)")
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// * Parsing of the part6.xml fails
    /// * The table element of the "Registry of DICOM Data Elements" chapter
    /// cannot be found
    /// * The format of how values are stored in part6.xml has changed and this
    /// function is no longer able to parse it appropriately
    pub fn parse_data_element_registry(&self) -> Result<Vec<DataElement>, Box<Error>> {
        let root = xmltree::Element::parse(self.part6_content.as_bytes())?;
        let chapter_6_table_body = match Self::find_chapter_table_body(&root, "6") {
            Some(element) => element,
            None => return Err(From::from("Unable to find chapter 6 table body.")),
        };

        Self::parse_data_elements(&chapter_6_table_body)
    }

    /// Returns all file meta elements defined in the "Registry of DICOM File
    /// Meta Elements" table of the DICOM standard.
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// * Parsing of the part6.xml fails
    /// * The table element of the "Registry of DICOM File Meta Elements"
    /// chapter cannot be found
    /// * The format of how values are stored in part6.xml has changed and this
    /// function is no longer able to parse it appropriately
    pub fn parse_file_meta_element_registry(&self) -> Result<Vec<DataElement>, Box<Error>> {
        let root = xmltree::Element::parse(self.part6_content.as_bytes())?;
        let chapter_7_table_body = match Self::find_chapter_table_body(&root, "7") {
            Some(element) => element,
            None => return Err(From::from("Unable to find chapter 7 table body.")),
        };

        Self::parse_data_elements(&chapter_7_table_body)
    }

    fn download_part_6() -> Result<String, Box<Error>> {
        let mut response = reqwest::get(
            "http://dicom.nema.org/medical/dicom/current/source/docbook/part06/part06.xml",
        )?;

        Self::read_content(&mut response).map_err(|e| e.into())
    }

    fn find_chapter_table_body<'a>(
        root: &'a xmltree::Element,
        chapter_name: &str,
    ) -> Option<&'a xmltree::Element> {
        for child in &root.children {
            if child.name == "chapter" {
                let label_attribute = match child.attributes.get("label") {
                    Some(a) => a,
                    None => continue,
                };
                if label_attribute == chapter_name {
                    for grand_child in &child.children {
                        if grand_child.name == "table" {
                            for grand_grand_child in &grand_child.children {
                                if grand_grand_child.name == "tbody" {
                                    return Some(grand_grand_child);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn parse_data_elements(table_body: &xmltree::Element) -> Result<Vec<DataElement>, Box<Error>> {
        let mut data_elements = Vec::new();

        // xml underneath chapter 6 tbody is <tr><td><para></para></td><td>...</tr>
        for tr in &table_body.children {
            let mut data_element = DataElement::new();
            let mut counter = 0;
            for td in &tr.children {
                let mut para = &td.children[0];
                assert!(para.name == "para");

                if !para.children.is_empty() && &para.children[0].name == "emphasis" {
                    // some text is italic and thus has an extra "emphasis" sub-element...
                    para = &para.children[0];
                }

                let text = para.text.clone();
                if text.is_none() && counter != 5 {
                    continue;
                }

                match counter {
                    0 => data_element.tag = text.unwrap(),
                    1 => data_element.name = text.unwrap(),
                    2 => {
                        let text = text.unwrap();
                        // some empty keywords with emphasis element have a CRLF as value...
                        // this seems to be parsed as "1"... we ignore it
                        if text != "1" {
                            data_element.keyword = text;
                        }
                    }
                    3 => {
                        let vr = text.unwrap();
                        // TODO: not too clean... tags like "Item" have the text "See Note 2" as VR
                        // Note 2 says that these tags do not have a VR
                        if !vr.starts_with("See Note") {
                            data_element.vr = vr;
                        }
                    }
                    4 => data_element.vm = text.unwrap(),
                    5 => data_element.comment = text,
                    _ => return Err(From::from("Found unexpected number of 'td' elements")),
                }

                counter += 1;
            }

            data_elements.push(data_element);
        }

        Ok(data_elements)
    }

    fn read_content<R: Read>(reader: &mut R) -> Result<String, ::std::io::Error> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }
}
