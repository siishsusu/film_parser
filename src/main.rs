use film_parser::*;
use colored::*;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("{}: No command provided. Use 'cargo run -- help' for usage information.", "Error".red().bold());
        return Ok(());
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
                eprintln!("{}: No file path provided. You must specify path to the file with information.", "Error".red().bold());
                return Ok(());
            }
            let filepath = &args[2];
            match read_lines(filepath) {
                Ok(lines) => {
                    parse_films(lines)?;
                }
                Err(e) => eprintln!(
                    "{}'{}': {}",
                    "Error reading file ".red().bold(),
                    filepath,
                    e
                ),
            }
        }
        _ => eprintln!("{}: Invalid command. Use 'cargo run -- help' for usage information.", "Error".red().bold()),
    }
    Ok(())
}

fn show_help(){
    println!("{}", "\nFilm Parser - A command-line tool for parsing film information from file.".green().bold().italic());
    println!("{}", "Commands:".green().bold());
    println!("{}  - Parse the specified file and display its content.", "\tparse <filename>".italic());
    println!("{}              - Show this help information.", "\thelp".italic());
    println!("{}           - Show credits information.", "\tcredits".italic());
    println!("{}", "\nExample usage:".green().bold());
    println!("{}", "\tcargo run -- parse data/film_info.txt".italic());
    println!("{}", "\tcargo run -- help".italic());
    println!("{}", "\tcargo run -- credits".italic());
}

fn show_credits() {
    println!("{}", "Film Parser v1.0".italic());
    println!("Developed by {}", "Rudas Vladyslava".bold());
    println!("{}", "Thanks for using the Film Parser CLI!".italic().yellow());
}