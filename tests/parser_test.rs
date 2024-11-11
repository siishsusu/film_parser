use film_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_film_parsing() {
        let input = "Title: I Used To Be Funny; Year: 2023; Director: Ally Pankiw;\
        Writer: Ally Pankiw; Genre: [Comedy, Drama]; Stars: [Rachel Sennott, Olga Petsa, Jason Jones];\
        Description: A stand-up comedian struggling with PTSD.";
        let film = Film::from_str(input).expect("Failed to parse valid film");

        assert_eq!(film.title, "I Used To Be Funny");
        assert_eq!(film.year, 2023);
        assert_eq!(film.director, "Ally Pankiw");
        assert_eq!(film.writer, "Ally Pankiw");
        assert_eq!(film.genre, vec!["Comedy".to_string(), "Drama".to_string()]);
        assert_eq!(film.stars, vec!["Rachel Sennott".to_string(), "Olga Petsa".to_string(), "Jason Jones".to_string()]);
        assert_eq!(film.description, "A stand-up comedian struggling with PTSD.");
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = Film::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Some fields may be missing");
    }

    #[test]
    fn test_missing_fields() {
        let input = "Title: Some_Title; Year: 2023;";
        let result = Film::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Some fields may be missing");
    }

    #[test]
    fn test_valid_title() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let film = Film::from_str(input).expect("Failed to parse valid title");
        assert_eq!(film.title, "Some_Title");
    }

    #[test]
    fn test_invalid_title() {
        let input = "Title: ; Year: 2024; Director: Some_Director;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let result = Film::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_year() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let film = Film::from_str(input).expect("Failed to parse valid year");
        assert_eq!(film.year, 2024);
    }

    #[test]
    fn test_invalid_year() {
        let input = "Title: Some_Title; Year: twenty twenty-three; Director: Some_Director;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let result = Film::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid year");
    }

    #[test]
    fn test_valid_director() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let film = Film::from_str(input).expect("Failed to parse valid director");
        assert_eq!(film.director, "Some_Director");
    }

    #[test]
    fn test_invalid_director() {
        let input = "Title: Some_Title; Year: 2024; Director: ;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let result = Film::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_writer() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let film = Film::from_str(input).expect("Failed to parse valid writer");
        assert_eq!(film.writer, "Some_Writer");
    }

    #[test]
    fn test_invalid_writer() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: ; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let result = Film::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_genre() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: ; Genre: [Drama, Mystery]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let film = Film::from_str(input).expect("Failed to parse valid genre");
        assert_eq!(film.genre, vec!["Drama".to_string(), "Mystery".to_string()]);
    }

    #[test]
    fn test_invalid_writer() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: ; Genre: Drama, Mystery; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let result = Film::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_stars() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: ; Genre: [Drama, Mystery]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let film = Film::from_str(input).expect("Failed to parse valid stars");
        assert_eq!(film.stars, vec!["Some_Actor_A".to_string(), "Some_Actor_B".to_string()]);
    }

    #[test]
    fn test_invalid_stars() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: ; Genre: [Drama, Mystery]; Stars: Some_Actor_A, Some_Actor_B;\
         Description: Some_Description.";
        let result = Film::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_description() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: Some_Description.";
        let film = Film::from_str(input).expect("Failed to parse valid writer");
        assert_eq!(film.description, "Some_Description");
    }

    #[test]
    fn test_invalid_description() {
        let input = "Title: Some_Title; Year: 2024; Director: Some_Director;\
         Writer: Some_Writer; Genre: [Some_Genre]; Stars: [Some_Actor_A, Some_Actor_B];\
         Description: ";
        let result = Film::from_str(input);
        assert!(result.is_err());
    }
}