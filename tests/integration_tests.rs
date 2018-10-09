extern crate dictionary_parser;

#[test]
fn test_parse_data_element_registry() {
    match dictionary_parser::parse_data_element_registry() {
        Ok(elements) => {
            // 1000 is pretty random... just checking that we have
            // successfully parsed quite a bit of data and don't want
            // to hard-code the current total number of data elements
            assert!(elements.len() > 1000);

            // checking some random data elements
            let length_to_end = &elements[0];
            assert_eq!(length_to_end.tag, "(0008,0001)");
            assert_eq!(length_to_end.name, "Length to End");
            assert_eq!(length_to_end.keyword, "LengthToEnd");
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
