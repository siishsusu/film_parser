//! A Rust application for parsing film information from raw text into a structured format.
//! This crate includes functionality to read, parse, and write film data such as title, director,
//! writer, genre, stars and description into a `Film` struct for easy access and manipulation.

use crate::FilmParserError::FileReadingError;
use pest::Parser;
use pest_derive::Parser;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::*;
use thiserror::Error;

/// Enum representing possible errors that can occur while using the film parser.
#[derive(Error, Debug)]
pub enum FilmParserError {
    /// Error: specified file was not found.
    #[error("Specified file was not found: {0}")]
    NoFileFound(String),

    /// Error: failed to read the file.
    #[error("Failed to read the file: {0}")]
    FileReadingError(String),

    /// Error: failed to open the file.
    #[error("Failed to open the file {0}")]
    FileOpeningError(String),

    /// Error: failed to create the file.
    #[error("Failed to create the file {0}")]
    FileCreatingError(String),

    /// Error: failed to write the file.
    #[error("Failed to write to the file {0}")]
    FileWritingError(String),

    /// Error: failed to parse the file content.
    #[error("Failed to parse the file content: {0}")]
    ParsingError(String),

    /// Error: failed to parse the rule content
    #[error("Failed to parse the rule {0} content: {1}")]
    RuleParsingError(String, String),

    /// Error: missing required film fields
    #[error("Missing required film fields")]
    MissingFieldsError,

    /// Error: unknown rule
    #[error("Unknown rule {0}")]
    UnknownRule(String),
}

/// Reads lines from a specified file.
pub fn read_lines(filename: &str) -> Result<Vec<String>, FilmParserError> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(FilmParserError::NoFileFound(filename.to_string()));
    }

    let file = File::open(filename)
        .map_err(|_| FilmParserError::FileOpeningError(filename.to_string()))?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines.map_err(|e| FileReadingError(format!("{}: {}", filename, e)))
}

#[derive(Parser)]
#[grammar = "film.pest"]
/// Grammar Rules
pub struct FilmParser;

/// Represents a film with structured data fields.
/// Each field captures a different piece of film information, such as the title, release year,
/// director, writer, genre, stars and description.
#[derive(Debug, Clone)]
pub struct Film {
    /// The title of the film.
    pub title: String,
    /// The release year of the film.
    pub year: u32,
    /// The director of the film.
    pub director: String,
    /// The writer of the film.
    pub writer: String,
    /// The genres associated with the film.
    pub genre: Vec<String>,
    /// The main cast of the film.
    pub stars: Vec<String>,
    /// A brief description of the film.
    pub description: String,
}

/// Creates a new `Film` instance.
impl Film {
    pub fn new(
        title: String,
        year: u32,
        director: String,
        writer: String,
        genre: Vec<String>,
        stars: Vec<String>,
        description: String,
    ) -> Self {
        Film {
            title,
            year,
            director,
            writer,
            genre,
            stars,
            description,
        }
    }

    fn parse_string_field(inner_pair: pest::iterators::Pair<Rule>, target_rule: Rule) -> String {
        inner_pair
            .into_inner()
            .find_map(|pair| {
                if pair.as_rule() == target_rule {
                    Some(pair.as_str().to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    fn parse_vector_field(inner_pair: pest::iterators::Pair<Rule>, list_rule: Rule) -> Vec<String> {
        inner_pair
            .into_inner()
            .find_map(|pair| {
                if pair.as_rule() == list_rule {
                    Some(
                        pair.into_inner()
                            .flat_map(|item| {
                                item.as_str()
                                    .trim_matches('"')
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .collect::<Vec<String>>()
                            })
                            .collect::<Vec<String>>(),
                    )
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    pub fn parse_to_struct(pair: pest::iterators::Pair<Rule>) -> Result<Self, FilmParserError> {
        if pair.as_str().trim().is_empty() {
            return Err(FilmParserError::ParsingError(
                "Empty input was provided".to_string(),
            ));
        }

        let mut title = String::new();
        let mut year = 0;
        let mut director = String::new();
        let mut writer = String::new();
        let mut genre = Vec::new();
        let mut stars = Vec::new();
        let mut description = String::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::film => {
                    for inner_pair_1 in inner_pair.into_inner() {
                        match inner_pair_1.as_rule() {
                            Rule::Title => {
                                title = Self::parse_string_field(inner_pair_1, Rule::title_value)
                            }
                            Rule::Year => {
                                if let Ok(parsed_year) =
                                    inner_pair_1.clone().into_inner().as_str().parse::<u32>()
                                {
                                    year = parsed_year;
                                } else {
                                    return Err(FilmParserError::RuleParsingError(
                                        "year".to_string(),
                                        format!("{:?}", inner_pair_1.as_rule()),
                                    ));
                                }
                            }
                            Rule::Director => {
                                director =
                                    Self::parse_string_field(inner_pair_1, Rule::director_value)
                            }
                            Rule::Writer => {
                                writer = Self::parse_string_field(inner_pair_1, Rule::writer_value)
                            }
                            Rule::Genre => {
                                genre = Self::parse_vector_field(inner_pair_1, Rule::genre_list)
                            }

                            Rule::Stars => {
                                stars = Self::parse_vector_field(inner_pair_1, Rule::stars_list)
                            }
                            Rule::Description => {
                                description =
                                    Self::parse_string_field(inner_pair_1, Rule::description_value)
                            }
                            _ => {
                                return Err(FilmParserError::UnknownRule(format!(
                                    "{:?}",
                                    inner_pair_1.as_rule()
                                )));
                            }
                        }
                    }
                }
                _ => {
                    return Err(FilmParserError::UnknownRule(format!(
                        "{:?}",
                        inner_pair.as_rule()
                    )));
                }
            }
        }

        if title.is_empty()
            || year == 0
            || director.is_empty()
            || writer.is_empty()
            || genre.is_empty()
            || stars.is_empty()
            || description.is_empty()
        {
            return Err(FilmParserError::MissingFieldsError);
        }

        Ok(Self::new(
            title,
            year,
            director,
            writer,
            genre,
            stars,
            description,
        ))
    }
}

/// Parses a list of film data strings into `Film` structs and writes results to files.
pub fn parse_films(films: Vec<String>) -> Result<Vec<Film>, FilmParserError> {
    let mut films_res = Vec::new();

    for film in films {
        let pairs = FilmParser::parse(Rule::file, &film)
            .map_err(|_| FilmParserError::ParsingError(film.clone()))?;

        for pair in pairs {
            match Film::parse_to_struct(pair) {
                Ok(parsed_film) => films_res.push(parsed_film),
                Err(err) => {
                    return Err(FilmParserError::ParsingError(format!(
                        "{} - {:?}",
                        film, err
                    )))
                }
            }
        }
    }

    write_films_to_file(films_res.clone(), "data/result_file.txt")?;
    write_films_to_file_as_structure_without_formating(
        films_res.clone(),
        "data/result_wo_formating_file.txt",
    )?;

    Ok(films_res)
}

/// Writes parsed data to specified file with formated string.
pub fn write_films_to_file(films: Vec<Film>, filename: &str) -> Result<(), FilmParserError> {
    let mut file = File::create(filename)
        .map_err(|_| FilmParserError::FileCreatingError(filename.to_string()))?;
    for film in films {
        writeln!(
            file,
            "Title: {}\nYear: {}\nDirector: {}\nWriter: {}\nGenre: {}\nStars: {}\nDescription: {}\n",
            film.title,
            film.year,
            film.director,
            film.writer,
            film.genre.join(", "),
            film.stars.join(", "),
            film.description
        )
            .map_err(|_| FilmParserError::FileWritingError(filename.to_string()))?;
    }

    Ok(())
}

/// Writes parsed data to specified file without formating.
pub fn write_films_to_file_as_structure_without_formating(
    films: Vec<Film>,
    filename: &str,
) -> Result<(), FilmParserError> {
    let mut file = File::create(filename)
        .map_err(|_| FilmParserError::FileCreatingError(filename.to_string()))?;

    for film in films {
        writeln!(file, "{:?}", film)
            .map_err(|_| FilmParserError::FileWritingError(filename.to_string()))?;
    }

    Ok(())
}
