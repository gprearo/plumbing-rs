use std::fs;
use crate::data::provider::DataProvider;

pub struct TextFileProvider {
    path: String
}

impl TextFileProvider {
    pub fn new(path: String) -> TextFileProvider {
        return TextFileProvider{path};
    }
}

impl DataProvider<String> for TextFileProvider {
    fn get_data(&self) -> String {
        return fs::read_to_string(&self.path).expect("Error reading file.");
    }
}