mod database;
mod directoryparser;
mod sourcefileparser;

fn main() {
    directoryparser::DirectoryParser::parse(&String::from("/home/demensdeum/Sources/Death-Mask"));
}
