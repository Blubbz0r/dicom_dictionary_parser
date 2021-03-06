use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use DataElement;
use Kind;
use UID;

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
        Ok(Self {
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
    pub fn with_part6_file(file_path: &Path) -> Result<Self, ::std::io::Error> {
        let mut file = File::open(file_path)?;
        Ok(Self {
            part6_content: Self::read_content(&mut file)?,
        })
    }

    /// Creates a new `Parser` instance given the full `contents` of a part6.xml file.
    pub fn with_part6_file_contents(contents: String) -> Self {
        Self {
            part6_content: contents,
        }
    }

    /// Returns all data elements defined in the "Registry of DICOM Data
    /// Elements" table of the DICOM standard.
    ///
    /// Note that **all** data elemnts from the dictionary are returned,
    /// including elements:
    ///
    /// * without name/keyword (e.g. "(0018,0061)")
    /// * with tags defining ranges (e.g. "EscapeTriplet" -> "(1000,xxx0)")
    /// * without VR (e.g. "Item" -> "(FFFE,E000)")
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// * Parsing of the part6.xml fails
    ///   * The table element of the "Registry of DICOM Data Elements" chapter
    /// cannot be found
    ///   * The format of how values are stored in part6.xml has changed and this
    /// function is no longer able to parse it appropriately
    pub fn parse_data_element_registry(&self) -> Result<Vec<DataElement>, Box<Error>> {
        self.parse_data_elements("6")
    }

    /// Returns all file meta elements defined in the "Registry of DICOM File
    /// Meta Elements" table of the DICOM standard.
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// * Parsing of the part6.xml fails
    ///   * The table element of the "Registry of DICOM File Meta Elements"
    /// chapter cannot be found
    ///   * The format of how values are stored in part6.xml has changed and this
    /// function is no longer able to parse it appropriately
    pub fn parse_file_meta_element_registry(&self) -> Result<Vec<DataElement>, Box<Error>> {
        self.parse_data_elements("7")
    }

    /// Returns all file meta elements defined in the "Registry of DICOM
    /// Directory Structuring Elements" table of the DICOM standard.
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// * Parsing of the part6.xml fails
    ///   * The table element of the "Registry of DICOM Directory Structuring
    /// Elements" chapter cannot be found
    ///   * The format of how values are stored in part6.xml has changed and this
    /// function is no longer able to parse it appropriately
    pub fn parse_directory_structuring_element_registry(
        &self,
    ) -> Result<Vec<DataElement>, Box<Error>> {
        self.parse_data_elements("8")
    }

    /// Returns all unique identifiers defined in the "Registry of DICOM Unique
    /// Identifiers (UIDs)" table of the DICOM standard.
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// * Parsing of the part6.xml fails
    ///   * The table element of the "Registry of DICOM Unique Identifiers
    /// (UIDs)" chapter cannot be found
    ///   * The format of how values are stored in part6.xml has changed and this
    /// function is no longer able to parse it appropriately
    pub fn parse_unique_identifier_registry(&self) -> Result<Vec<UID>, Box<Error>> {
        let root = xmltree::Element::parse(self.part6_content.as_bytes())?;
        let chapter_a_table_body = match Self::find_chapter_table_body(&root, "A") {
            Some(element) => element,
            None => return Err(From::from("Unable to find chapter 'A' table body.")),
        };

        let mut uids = Vec::new();

        // xml underneath chapter tbody is <tr><td><para></para></td><td>...</tr>
        for tr in &chapter_a_table_body.children {
            let mut uid = UID::new();
            let mut counter = 0;
            for td in &tr.children {
                let mut para = &td.children[0];
                assert!(para.name == "para");

                if !para.children.is_empty() && &para.children[0].name == "emphasis" {
                    // some text is italic and thus has an extra "emphasis" sub-element...
                    para = &para.children[0];
                }

                let text = para.text.clone();

                match counter {
                    0 => {
                        uid.value = text.unwrap();

                        // values in "UID Value" column contain zero-width spaces...
                        // we'll trim them out
                        uid.value = uid.value.replace("\u{200b}", "");
                    }
                    1 => {
                        uid.full_name = text.unwrap();
                        uid.normalized_name = Self::normalize_uid_name(&uid.full_name);
                    }
                    2 => match text.unwrap().as_ref() {
                        "Application Context Name" => uid.kind = Kind::ApplicationContextName,
                        "Application Hosting Model" => uid.kind = Kind::ApplicationHostingModel,
                        "Coding Scheme" => uid.kind = Kind::CodingScheme,
                        "DICOM UIDs as a Coding Scheme" => uid.kind = Kind::DicomUidsAsCodingScheme,
                        "LDAP OID" => uid.kind = Kind::LdapOid,
                        "Mapping Resource" => uid.kind = Kind::MappingResource,
                        "Meta SOP Class" => uid.kind = Kind::MetaSopClass,
                        "Service Class" => uid.kind = Kind::ServiceClass,
                        "SOP Class" => uid.kind = Kind::SopClass,
                        "Synchronization Frame of Reference" => {
                            uid.kind = Kind::SynchronizationFrameOfReferences
                        }
                        "Transfer Syntax" => uid.kind = Kind::TransferSyntax,
                        "Well-known frame of reference" => {
                            uid.kind = Kind::WellKnownFrameOfReference
                        }
                        "Well-known Printer SOP Instance" => {
                            uid.kind = Kind::WellKnownPrinterSopInstance
                        }
                        "Well-known Print Queue SOP Instance" => {
                            uid.kind = Kind::WellKnownPrintQueueSopInstance
                        }
                        "Well-known SOP Instance" => uid.kind = Kind::WellKnownSopInstance,
                        val @ _ => return Err(From::from(format!("Unknown UID type '{}'", val))),
                    },
                    3 => { /* "Part" column, which we ignore right now */ }
                    _ => return Err(From::from("Found unexpected number of 'td' elements")),
                }

                counter += 1;
            }

            uids.push(uid);
        }

        Ok(uids)
    }

    fn download_part_6() -> Result<String, Box<Error>> {
        let mut response = reqwest::get(
            "http://dicom.nema.org/medical/dicom/current/source/docbook/part06/part06.xml",
        )?;

        Self::read_content(&mut response).map_err(|e| e.into())
    }

    fn parse_data_elements(&self, chapter_label: &str) -> Result<Vec<DataElement>, Box<Error>> {
        let root = xmltree::Element::parse(self.part6_content.as_bytes())?;
        let chapter_table_body = match Self::find_chapter_table_body(&root, chapter_label) {
            Some(element) => element,
            None => {
                return Err(From::from(format!(
                    "Unable to find chapter {} table body.",
                    chapter_label
                )))
            }
        };

        let mut data_elements = Vec::new();

        // xml underneath chapter tbody is <tr><td><para></para></td><td>...</tr>
        for tr in &chapter_table_body.children {
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

                // name, keyword, vr and/or vm is empty for a handful of elements...
                if text.is_none() && counter != 5 {
                    counter += 1;
                    continue;
                }

                match counter {
                    0 => data_element.tag = text.unwrap(),
                    1 => data_element.name = text.unwrap(),
                    2 => data_element.keyword = text.unwrap(),
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

    fn read_content<R: Read>(reader: &mut R) -> Result<String, ::std::io::Error> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        Ok(content)
    }

    fn normalize_uid_name(full_uid_name: &str) -> String {
        let mut normalized_uid_name = full_uid_name.to_owned();
        if normalized_uid_name.contains(":") {
            let colon_index = normalized_uid_name.find(":").unwrap();
            normalized_uid_name.split_off(colon_index);
            normalized_uid_name.shrink_to_fit();
        }

        if normalized_uid_name.contains(" (Retired)") {
            normalized_uid_name = normalized_uid_name.replace(" (Retired)", "");
        }

        normalized_uid_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_uid_name_doesnt_change_input_without_colon_or_retired() {
        assert_eq!(
            Parser::normalize_uid_name("Test String"),
            "Test String".to_owned()
        );
    }

    #[test]
    fn normalize_uid_name_strips_everything_starting_at_colon() {
        assert_eq!(
            Parser::normalize_uid_name("Test String: With a colon"),
            "Test String".to_owned()
        );
    }

    #[test]
    fn normalize_uid_name_removes_retired() {
        assert_eq!(
            Parser::normalize_uid_name("Test String (Retired)"),
            "Test String".to_owned()
        );
    }
}
