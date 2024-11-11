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
    let lines: Result<Vec<String>, std::io::Error> = reader.lines().collect();
    lines.map_err(|e| anyhow!("Failed to read the file: {}: {}", filename, e))
}

#[derive(Parser)]
#[grammar = "film.pest"]
pub struct FilmParser;

#[derive(Debug, Clone)]
pub struct Film {
    title: String,
    year: u32,
    director: String,
    writer: String,
    genre: Vec<String>,
    stars: Vec<String>,
    description: String,
}

impl Film {
    pub fn new(title: String, year: u32, director: String, writer: String, genre: Vec<String>, stars: Vec<String>, description: String) -> Self {
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

    fn parse_to_struct(pair: pest::iterators::Pair<Rule>) -> Option<Self> {
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
                                title = inner_pair_1.into_inner()
                                    .find_map(|inner_inner_pair| {
                                        if let Rule::title_value = inner_inner_pair.as_rule() {
                                            Some(inner_inner_pair.as_str().to_string())
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or_else(|| String::new());
                            }
                            Rule::Year => {
                                let year_str = inner_pair_1.into_inner().as_str();
                                match year_str.parse::<u32>() {
                                    Ok(parsed_year) => {
                                        year = parsed_year;
                                    }
                                    Err(_) => {
                                        eprintln!("Failed to parse year: {}", year_str);
                                        year = 0;
                                    }
                                }
                            }
                            Rule::Director => {
                                director = inner_pair_1.into_inner()
                                    .find_map(|inner_inner_pair| {
                                        if let Rule::director_value = inner_inner_pair.as_rule() {
                                            Some(inner_inner_pair.as_str().to_string())
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or_else(|| String::new());
                            }
                            Rule::Writer => {
                                writer = inner_pair_1.into_inner()
                                    .find_map(|inner_inner_pair| {
                                        if let Rule::writer_value = inner_inner_pair.as_rule() {
                                            Some(inner_inner_pair.as_str().to_string())
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or_else(|| String::new());
                            }
                            Rule::Genre => {
                                genre = inner_pair_1.into_inner()
                                    .find_map(|inner_inner_pair| {
                                        if let Rule::genre_list = inner_inner_pair.as_rule() {
                                            Some(inner_inner_pair.into_inner()
                                                .map(|g| g.as_str().trim_matches('"').to_string())
                                                .flat_map(|g| g.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>())
                                                .collect::<Vec<String>>())
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or_else(|| Vec::new());
                            },

                            Rule::Stars => {
                                stars = inner_pair_1.into_inner()
                                    .find_map(|inner_inner_pair| {
                                        if let Rule::stars_list = inner_inner_pair.as_rule() {
                                            Some(inner_inner_pair.into_inner()
                                                .map(|star| star.as_str().trim_matches('"').to_string())
                                                .flat_map(|star| star.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>())
                                                .collect::<Vec<String>>())
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or_else(|| Vec::new());
                            }
                            Rule::Description => {
                                description = inner_pair_1.into_inner()
                                    .find_map(|inner_inner_pair| {
                                        if let Rule::description_value = inner_inner_pair.as_rule() {
                                            Some(inner_inner_pair.as_str().to_string())
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or_else(|| String::new());
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

        Some(Self::new(
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
            if let Some(film) = Film::parse_to_struct(pair) {
                films_res.push(film);
            } else {
                eprintln!("Failed to parse film: {}", film);
            }
        }
    }

    Ok(films_res)
}