extern crate dicom_dictionary_parser as dict_parser;

#[test]
fn test_parse_data_element_registry() {
    let parser = dict_parser::Parser::new().unwrap();
    match parser.parse_data_element_registry() {
        Ok(elements) => {
            // 1000 is pretty random... just checking that we have
            // successfully parsed quite a bit of data and don't want
            // to hard-code the current total number of data elements
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

            // TODO: add test for special cases ("See Note", "<emphasis> sub element", "1" hack)
        }
        Err(e) => assert!(false, e.to_string()),
    }
}

#[test]
fn test_parse_file_meta_element_registry() {
    let parser = dict_parser::Parser::new().unwrap();
    match parser.parse_file_meta_element_registry() {
        Ok(elements) => {
            // 10 is pretty random... just checking that we have
            // successfully parsed quite a bit of data and don't want
            // to hard-code the current total number of file meta elements
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
