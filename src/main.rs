use std::env;
use std::process::exit;
mod database;
mod directoryparser;
mod sourcefileparser;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Incorrect arguments, must be: hangar18 \"/home/demensdeum/Sources/sourceCode\" \"searchDefinition\" ");
        exit(1);
    }

    let parseResults = directoryparser::DirectoryParser::parse(&args[1]);
    let mut database = database::Database::new();
    database.add(parseResults);
    let grepOutput = database.grepOutputFor(&args[2]);
    if grepOutput.is_none() {
        println!("No match");
        exit(2);
    }
    println!("{}", grepOutput.unwrap());

    exit(0);
}
