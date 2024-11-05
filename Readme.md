# Film Parser

## Project Name
**Film Parser**: A Rust application to parse film information from formatted strings into structured data.

## Overview
This project implements a Rust application that parses a string containing film information into a structured format defined by the `Film` struct. The goal is to facilitate the extraction and manipulation of film data, making it easier to work with various film-related tasks.

## Technical Description
The parsing process involves taking a string formatted in a specific way (for example,
```text
"Title: I Used To Be Funny; Year: 2023; Director: Ally Pankiw; Writer: Ally Pankiw; Genre: [Comedy, Drama]; Stars: [Rachel Sennott, Olga Petsa, Jason Jones]; Description: Sam, a stand-up comedian struggling with PTSD, weighs whether or not to join the search for a missing teenage girl she used to nanny."
```

### Parsing Steps
1. **Input File**: The input is a file containing strings with film details, where each string is separated by semicolons.
2. **Reading File**: The file is read line by line, where each line represents a separate film's information.
3. **Splitting**: Each line is split into key-value pairs using the semicolon as a delimiter.
4. **Trimming**: Each key-value pair is trimmed to remove extra spaces.
5. **Mapping to Struct**: The resulting pairs are mapped to the fields of the Film struct.

### Film Struct
The `Film` struct will be defined as follows:
```rust
struct Film {
    title: String,
    year: u32,
    director: String,
    writer: String,
    genre: Vec<String>,
    stars: Vec<String>,
    description: String,
}
```

## Usage
Once parsed, the resulting Film struct can be used for various purposes, including displaying film details, storing them in a database, or further processing them in an application.