use std::fs;
use crate::data::provider::DataProvider;

pub struct TextFileProvider<'a> {
    path: &'a String
}

impl<'a> TextFileProvider<'a> {
    pub fn new(path: &'a String) -> TextFileProvider<'a> {
        return TextFileProvider{path};
    }
}

impl<'a> DataProvider<String> for TextFileProvider<'a> {
    fn get_data(&self) -> String {
        return fs::read_to_string(self.path).expect("Error reading file.");
    }
}