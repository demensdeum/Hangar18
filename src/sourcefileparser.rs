use std::fs::File;
use std::io::Read;

extern crate regex;
use self::regex::Regex;

#[derive(Debug)]
pub struct SourceFileParserResult {
    pub result_type: String,
    pub value: String,
    pub file_path: String,
    pub line_index: usize
}

pub struct SourceFileParser {

}

impl SourceFileParser {

    pub fn parse(file_path: &String) -> Vec<SourceFileParserResult> {
        //println!("In file {}", file_path);

        let mut file = File::open(file_path.clone()).expect("file not found");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("something went wrong reading the file");

        let lines = contents.split("\n");

        let mut results = Vec::new();

        for (line_index, line) in lines.enumerate() {
            let class_parse_result = SourceFileParser::parse_class_declaration(String::from(line),
                                                                              file_path,
                                                                              line_index + 1);
            if !class_parse_result.is_none() {
                results.push(class_parse_result.unwrap());
            }
            else {

                let method_parse_result = SourceFileParser::parse_method_declaration(String::from(line),
                                                                                     file_path,
                                                                                     line_index + 1);
                if !method_parse_result.is_none() {
                    results.push(method_parse_result.unwrap());
                }
            }
        }

        results
    }

    fn parse_class_declaration(line: String,
                               file_path: &String,
                               line_index: usize) -> Option<SourceFileParserResult> {

        let class_declaration_parser_regex = Regex::new("class ([a-zA-Z]*)")
            .expect("wrong regexp for class parsing");

        let last_character = line.chars().last();
        if last_character.is_none() { return None; }
        if last_character.unwrap() == ';' { return None; } // skip class forward declaration;
        let line_string = &line.to_string();
        let captures = class_declaration_parser_regex.captures(line_string);
        if captures.is_none() { return None; }
        //println!("Captured class definition: {:?} line: {} line: {}", captures, line_index, line);

        let result = SourceFileParserResult {
            result_type: String::from("Class declaration"),
            value: String::from("value"),
            file_path: file_path.clone(),
            line_index: line_index};

        Some(result)
    }

    fn parse_method_declaration(line: String,
                                file_path: &String,
                                line_index: usize) -> Option<SourceFileParserResult>  {

        let class_declaration_parser_regex = Regex::new(".* ([a-zA-Z]*)\\(")
            .expect("wrong regexp for method parsing");

        let line_string = &line.to_string();
        let captures = class_declaration_parser_regex.captures(line_string);
        if captures.is_none() { return None; }
        let capturesResult = captures.unwrap();
        if capturesResult.len() < 2 { return None; }
        //println!("Captured method definition: {:?} line: {} line: {}", capturesResult, line_index, line);

        let result = SourceFileParserResult {
            result_type: String::from("Method declaration"),
            value: capturesResult[1].to_string(),
            file_path: file_path.clone(),
            line_index: line_index};

        Some(result)
    }
}