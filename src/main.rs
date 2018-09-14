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

    let parse_results = directoryparser::DirectoryParser::parse(&args[1]);
    let mut database = database::Database::new();
    database.add(parse_results);
    let grep_output = database.grep_output_for(&args[2]);
    if grep_output.is_none() {
        println!("No match");
        exit(2);
    }
    println!("{}", grep_output.unwrap());
    database.remove_all();

    exit(0);
}
