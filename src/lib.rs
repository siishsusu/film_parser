use std::fs::{File};
use std::path::Path;
use std::*;
use std::io::{BufRead, BufReader};
use anyhow::{anyhow, Context};
use pest::Parser;
use pest_derive::Parser;
use std::io::Write;

pub fn read_lines(filename: &str) -> anyhow::Result<Vec<String>> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(anyhow!("No file found: {:?}", path));
    }

    let file = File::open(filename)
        .with_context(|| format!("Failed to open the file {}", filename))?;
    let reader = BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    lines.map_err(|e| anyhow!("Failed to read the file: {}: {}", filename, e))
}

#[derive(Parser)]
#[grammar = "film.pest"]
pub struct FilmParser;

#[derive(Debug, Clone)]
pub struct Film {
    pub title: String,
    pub year: u32,
    pub director: String,
    pub writer: String,
    pub genre: Vec<String>,
    pub stars: Vec<String>,
    pub description: String,
}

impl Film {
    pub fn new(title: String,
               year: u32,
               director: String,
               writer: String,
               genre: Vec<String>,
               stars: Vec<String>,
               description: String) -> Self {
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
        inner_pair.into_inner()
            .find_map(|pair| if pair.as_rule() == target_rule { Some(pair.as_str().to_string()) } else { None })
            .unwrap_or_default()
    }

    fn parse_vector_field(inner_pair: pest::iterators::Pair<Rule>, list_rule: Rule) -> Vec<String> {
        inner_pair.into_inner()
            .find_map(|pair| if pair.as_rule() == list_rule {
                Some(pair.into_inner()
                    .flat_map(|item| item.as_str().trim_matches('"').split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>())
                    .collect::<Vec<String>>())
            } else {
                None
            })
            .unwrap_or_default()
    }

    pub fn parse_to_struct(pair: pest::iterators::Pair<Rule>) -> anyhow::Result<Self> {
        if pair.as_str().trim().is_empty() {
            return Err(anyhow!("Unexpected empty input provided"));
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
                                if let Ok(parsed_year) = inner_pair_1.clone().into_inner().as_str().parse::<u32>() {
                                    year = parsed_year;
                                } else {
                                    eprintln!("Failed to parse year: {}", inner_pair_1.as_str());
                                }
                            }
                            Rule::Director => {
                                director = Self::parse_string_field(inner_pair_1, Rule::director_value)
                            }
                            Rule::Writer => {
                                writer = Self::parse_string_field(inner_pair_1, Rule::writer_value)
                            }
                            Rule::Genre => {
                                genre = Self::parse_vector_field(inner_pair_1, Rule::genre_list)
                            },

                            Rule::Stars => {
                                stars = Self::parse_vector_field(inner_pair_1, Rule::stars_list)
                            }
                            Rule::Description => {
                                description = Self::parse_string_field(inner_pair_1, Rule::description_value)
                            }
                            _ => {
                                println!("Unknown rule inside film: {:?}", inner_pair_1.as_rule());
                            }
                        }
                    }
                }
                _ => {
                    println!("Unknown rule: {:?}", inner_pair.as_rule());
                }
            }
        }

        if title.is_empty() || year == 0 || director.is_empty() || writer.is_empty() ||
            genre.is_empty() || stars.is_empty() || description.is_empty() {
            return Err(anyhow!("Some fields may be missing"));
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

pub fn parse_films(films: Vec<String>) -> anyhow::Result<Vec<Film>> {
    let mut films_res = Vec::new();

    for film in films {
        let pairs = match FilmParser::parse(Rule::file, &film) {
            Ok(pairs) => pairs,
            Err(err) => {
                eprintln!("Failed to parse line: {} with error: {}", film, err);
                continue;
            }
        };

        for pair in pairs {
            match Film::parse_to_struct(pair) {
                Ok(parsed_film) => films_res.push(parsed_film),
                Err(err) => eprintln!("Error parsing film: {} - {}", film, err),
            }
        }
    }

    write_films_to_file(films_res.clone(), "data/result_file.txt")?;

    Ok(films_res)
}

pub fn write_films_to_file(films: Vec<Film>, filename: &str) -> anyhow::Result<()> {
    let mut file = File::create(filename)
        .with_context(|| format!("Failed to create file: {}", filename))?;

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
            .with_context(|| format!("Failed to write to file: {}", filename))?;
    }

    Ok(())
}