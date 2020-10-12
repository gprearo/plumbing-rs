use std::io::prelude::*;
use std::fs::File;
use std::fs;
use plumbing::data::provider::{TextFileProvider, DataProvider};

#[test]
pub fn text_file_provider_test() {
    let path = "text_file_provider_teste_data";
    let mut file = File::create(path).unwrap();
    let data = "Test data";
    let _ = file.write_all(data.as_bytes());

    let text_provider = TextFileProvider::new(String::from(path)); 
    let provided = text_provider.get_data();    
    assert_eq!(provided, data);

    let _ = fs::remove_file(path);

}