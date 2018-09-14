use std::collections::HashMap;
use self::super::sourcefileparser::SourceFileParserResult;

pub struct Database {

    pub database_results: HashMap<String, SourceFileParserResult>

}

impl Database {

    pub fn new() -> Database {
        return Database {
            database_results: HashMap::new()
        };
    }

    pub fn add(&mut self, results: Vec<SourceFileParserResult>) {

        for result in results {
            self.database_results.insert(result.value.clone(), result);
        }

        //println!("database_results: {:?}", self.database_results);
    }

    pub fn removeAll(&mut self) {
        self.database_results.clear();
    }

    pub fn grepOutputFor(&self, searchDefinition: &String) -> Option<String> {

        let result = self.database_results.get(searchDefinition);
        if result.is_none() {
            return None;
        }
        let actualResult = result.unwrap();

        let file_path = actualResult.file_path.clone();
        let line_index = actualResult.line_index.clone();

        Some(format!("{0}:{1}", file_path, line_index))
    }
}