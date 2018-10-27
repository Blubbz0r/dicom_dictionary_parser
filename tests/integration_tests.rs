extern crate dicom_dictionary_parser as dict_parser;

// Note: this contains tests against the "part06.xml" file that this lib was
// coded against as well as tests against a downloaded version of part 6 to
// prove that it still works for the current format.

fn parser_from_file() -> dict_parser::Parser {
    let part6_contents = include_bytes!("part06.xml");
    dict_parser::Parser::with_part6_file_contents(
        String::from_utf8_lossy(part6_contents).to_string(),
    )
}

#[test]
fn parse_data_element_registry_from_file() {
    let parser = parser_from_file();
    match parser.parse_data_element_registry() {
        Ok(elements) => {
            assert_eq!(elements.len(), 4266);

            let item_delimitation_item = &elements[4264];
            assert_eq!(item_delimitation_item.tag, "(FFFE,E00D)");
            assert_eq!(item_delimitation_item.name, "Item Delimitation Item");
            assert_eq!(
                item_delimitation_item.keyword,
                "Item\u{200b}Delimitation\u{200b}Item"
            );
            assert_eq!(item_delimitation_item.vr, "");
            assert_eq!(item_delimitation_item.vm, "1");
            assert!(item_delimitation_item.comment.is_none());

            let escape_triplet = &elements[3298];
            assert_eq!(escape_triplet.tag, "(1000,xxx0)");

            let unnamed_element = &elements[537];
            assert_eq!(unnamed_element.tag, "(0018,0061)");
            assert!(unnamed_element.name.is_empty());
            assert!(unnamed_element.keyword.is_empty());
            assert_eq!(unnamed_element.vr, "DS");
            assert_eq!(unnamed_element.vm, "1");
            assert_eq!(unnamed_element.comment, Some("RET".to_string()));
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn parse_file_meta_element_registry_from_file() {
    let parser = parser_from_file();
    match parser.parse_file_meta_element_registry() {
        Ok(elements) => {
            assert_eq!(elements.len(), 12);

            let transfer_syntax_uid = &elements[4];
            assert_eq!(transfer_syntax_uid.tag, "(0002,0010)");
            assert_eq!(transfer_syntax_uid.name, "Transfer Syntax UID");
            assert_eq!(
                transfer_syntax_uid.keyword,
                "Transfer\u{200b}Syntax\u{200b}UID"
            );
            assert_eq!(transfer_syntax_uid.vr, "UI");
            assert_eq!(transfer_syntax_uid.vm, "1");
            assert!(transfer_syntax_uid.comment.is_none());
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn parse_data_element_registry_from_downloaded_dict() {
    let parser = dict_parser::Parser::new().unwrap();
    match parser.parse_data_element_registry() {
        Ok(elements) => {
            // 1000 is pretty random... just checking that we have
            // successfully parsed quite a bit of data. exact test
            // is done against an actual xml file above
            assert!(elements.len() > 1000);

            // checking some random data elements
            let length_to_end = &elements[0];
            assert_eq!(length_to_end.tag, "(0008,0001)");
            assert_eq!(length_to_end.name, "Length to End");
            assert_eq!(length_to_end.keyword, "Length\u{200b}To\u{200b}End");
            assert_eq!(length_to_end.vr, "UL");
            assert_eq!(length_to_end.vm, "1");
            assert_eq!(length_to_end.comment, Some("RET".to_string()));

            let specific_character_set = &elements[1];
            assert_eq!(specific_character_set.tag, "(0008,0005)");
            assert_eq!(specific_character_set.vm, "1-n");
            assert!(specific_character_set.comment.is_none());
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn parse_file_meta_element_registry_from_downloaded_dict() {
    let parser = dict_parser::Parser::new().unwrap();
    match parser.parse_file_meta_element_registry() {
        Ok(elements) => {
            // 10 is pretty random... just checking that we have
            // successfully parsed quite a bit of data. exact test
            // is done against an actual xml file above
            assert!(elements.len() > 10);

            // checking some random file meta elements
            let file_meta_information_group_length = &elements[0];
            assert_eq!(file_meta_information_group_length.tag, "(0002,0000)");
            assert_eq!(
                file_meta_information_group_length.name,
                "File Meta Information Group Length"
            );
            assert_eq!(
                file_meta_information_group_length.keyword,
                "File\u{200b}Meta\u{200b}Information\u{200b}Group\u{200b}Length"
            );
            assert_eq!(file_meta_information_group_length.vr, "UL");
            assert_eq!(file_meta_information_group_length.vm, "1");
            assert!(file_meta_information_group_length.comment.is_none());
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn parse_directory_structuring_elements_from_file() {
    let parser = parser_from_file();
    match parser.parse_directory_structuring_elements() {
        Ok(elements) => {
            assert_eq!(elements.len(), 19);

            let item_delimitation_item = &elements[5];
            assert_eq!(item_delimitation_item.tag, "(0004,1212)");
            assert_eq!(item_delimitation_item.name, "File-set Consistency Flag");
            assert_eq!(
                item_delimitation_item.keyword,
                "File\u{200b}Set\u{200b}Consistency\u{200b}Flag"
            );
            assert_eq!(item_delimitation_item.vr, "US");
            assert_eq!(item_delimitation_item.vm, "1");
            assert!(item_delimitation_item.comment.is_none());
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn parse_directory_structuring_elements_from_downloaded_dict() {
    let parser = dict_parser::Parser::new().unwrap();
    match parser.parse_directory_structuring_elements() {
        Ok(elements) => {
            // 10 is pretty random... just checking that we have
            // successfully parsed quite a bit of data. exact test
            // is done against an actual xml file above
            assert!(elements.len() > 10);

            // checking some random element
            let file_set_id = &elements[0];
            assert_eq!(file_set_id.tag, "(0004,1130)");
            assert_eq!(file_set_id.name, "File-set ID");
            assert_eq!(file_set_id.keyword, "File\u{200b}Set\u{200b}ID");
            assert_eq!(file_set_id.vr, "CS");
            assert_eq!(file_set_id.vm, "1");
            assert!(file_set_id.comment.is_none());
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn parse_unique_identifiers_from_file() {
    let parser = parser_from_file();
    match parser.parse_unique_identifiers() {
        Ok(uids) => {
            assert_eq!(uids.len(), 400);

            let explicit_vr_little_endian = &uids[2];
            assert_eq!(explicit_vr_little_endian.value, "1.2.840.10008.1.2.1");
            assert_eq!(explicit_vr_little_endian.name, "Explicit VR Little Endian");
            assert_eq!(
                explicit_vr_little_endian.uid_type,
                dict_parser::UIDType::TransferSyntax
            );
        }
        Err(e) => assert!(false, e.to_string()),
    }
}
