use film_parser::*;
use colored::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("{}{}", "Error".red().bold(), ": No command provided. Use 'cargo run -- help' for usage information.");
        return;
    }

    match args[1].as_str(){
        "help" => {
            show_help()
        }
        "credits" => {
            show_credits()
        }
        "parse" => {
            if args.len() <= 2 {
                eprintln!("{}{}", "Error".red().bold(), ": No file path provided. You must specify path to the file with information.");
                return;
            }
            let filepath = &args[2];
            match read_lines(filepath) {
                Ok(lines) => {
                    println!("Successfully read {} lines from file '{}'.", lines.len().to_string().green(), filepath.green());
                    println!("{:?}", lines.get(0));
                }
                Err(e) => eprintln!(
                    "{}{} '{}': {}",
                    "Error".red().bold(),
                    " reading file ",
                    filepath,
                    e
                ),
            }
        }
        _ => eprintln!("{}{}", "Error".red().bold(), ": Invalid command. Use 'cargo run -- help' for usage information."),
    }
}


fn show_help(){
    println!("{}", "\nFilm Parser - A command-line tool for parsing film information from file.".green().bold().italic());
    println!("{}", "Commands:".green().bold());
    println!("{}  - {}", "\tparse <filename>".italic(), "Parse the specified file and display its content.");
    println!("{}              - {}", "\thelp".italic(), "Show this help information.");
    println!("{}           - {}", "\tcredits".italic(), "Show credits information.");
    println!("{}", "\nExample usage:".green().bold());
    println!("{}", "\tcargo run -- parse data/film_info.txt".italic());
    println!("{}", "\tcargo run -- help".italic());
    println!("{}", "\tcargo run -- credits".italic());
}

fn show_credits() {
    println!("{}", "Film Parser v1.0".italic());
    println!("{} {}", "Developed by", "Rudas Vladyslava".bold());
    println!("{}", "Thanks for using the Film Parser CLI!".italic().yellow());
}