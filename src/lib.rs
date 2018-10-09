extern crate reqwest;
extern crate xmltree;

use std::error::Error;
use std::io::Read;

#[derive(Debug)]
pub struct DataElement {
    pub tag: String,
    pub name: String,
    pub keyword: String,
    pub vr: String,
    pub vm: String,
    pub comment: Option<String>,
}

impl DataElement {
    pub fn new() -> DataElement {
        DataElement {
            tag: String::new(),
            name: String::new(),
            keyword: String::new(),
            vr: String::new(),
            vm: String::new(),
            comment: None,
        }
    }
}

pub fn parse_data_element_registry() -> Result<Vec<DataElement>, Box<Error>> {
    let document = download_part_6()?;

    let mut data_elements = Vec::new();

    let root = xmltree::Element::parse(document.as_bytes())?;
    let chapter_6_table_body = match find_chapter_6_table_body(&root) {
        Some(element) => element,
        None => return Err(From::from("Unable to find chapter 6 table body.")),
    };

    // xml underneath chapter 6 tbody is <tr><td><para></para></td><td>...</tr>
    for tr in &chapter_6_table_body.children {
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
                2 => data_element.keyword = text.unwrap().replace("\u{200b}", ""), // keyword have zero-width spaces in them...
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

fn download_part_6() -> Result<String, Box<Error>> {
    let mut response = reqwest::get(
        "http://dicom.nema.org/medical/dicom/current/source/docbook/part06/part06.xml",
    )?;
    let mut content = String::new();
    response.read_to_string(&mut content)?;
    Ok(content)
}

fn find_chapter_6_table_body(root: &xmltree::Element) -> Option<&xmltree::Element> {
    for child in &root.children {
        if child.name == "chapter" {
            let label_attribute = match child.attributes.get("label") {
                Some(a) => a,
                None => continue,
            };
            if label_attribute == "6" {
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
