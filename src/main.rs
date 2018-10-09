extern crate dictionary_parser;

fn main() {
    match dictionary_parser::parse_data_element_registry() {
        Ok(ref elements) => for element in elements {
            println!("{:?}", element)
        },
        Err(e) => eprintln!("{}", e),
    };
}
