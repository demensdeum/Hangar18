extern crate walkdir;
use self::walkdir::{DirEntry, WalkDir};

use self::super::sourcefileparser::*;

pub struct DirectoryParser {

}

impl DirectoryParser {

    pub fn parse(directory_path: &String) -> Vec<SourceFileParserResult> {

        let mut results = Vec::new();

        let files = DirectoryParser::collect_files(directory_path);
        for file in files {
            let parsed = SourceFileParser::parse(&file);
            for parsed_result in parsed {
                results.push(parsed_result)
            }
        }

        return results
    }

    fn collect_files(directory_path: &String) -> Vec<String> {

        let mut results = Vec::new();

        fn is_hidden(entry: &DirEntry) -> bool {
            entry.file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        }

        fn is_header_file(entry: &DirEntry) -> bool {
            entry.file_name()
                .to_str()
                .map(|s| s.ends_with(".h"))
                .unwrap_or(false)
        }

        let walker = WalkDir::new(directory_path).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry_result = &entry.unwrap();
            if is_header_file(entry_result) {
                //println!("{}", entry_result.path().display());
                results.push(String::from(entry_result.path().to_str().unwrap()));
            }
        }

        results
    }

}
