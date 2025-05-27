use std::{collections::HashMap, fmt};

use nom::{
    bytes::complete::{take_till, take_until, take_while}, multi::{many, many0}, sequence::{pair, preceded}, IResult, Parser
};
use regex::Regex;

pub fn parse(input: &str) -> String {
    let casing = find_casing(input).unwrap();

    casing.parse(input)
}

#[derive(Debug, Clone)]
struct CasingParseError<'a> {
    failed_str: &'a str,
}

impl fmt::Display for CasingParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Could not determine case for string {0}",
            self.failed_str
        )
    }
}

impl CasingParseError<'_> {
    pub fn new(failed_str: &str) -> CasingParseError<'_> {
        CasingParseError { failed_str }
    }
}

enum Casing {
    PascalCase,
    SnakeCase,
    TitleSnakeCase,
    ScreamingSnakeCase,
    LowerCase,
}

impl Casing {
    pub fn parse<'a>(&'a self, input: &'a str) -> String {
        match self {
            Casing::PascalCase => Self::parse_pascal(input),
            Casing::SnakeCase => Self::parse_snake(input),
            Casing::TitleSnakeCase => Self::parse_title_snake(input),
            Casing::ScreamingSnakeCase => Self::parse_screaming_snake(input),
            Casing::LowerCase => Self::parse_lower(input),
        }
    }

    fn parse_pascal(input: &str) -> String {
        // let (_, words) = many0((
        //     till_upper_case,
        //     while_lower_case,
        // )).parse(input).unwrap();
        let (_, a) = many(
            0..=input.len(),
            word_starting_with_upper,
        )
        .parse(input)
        .unwrap();

        a

        // words
        //     .iter()
        //     // .map(|w| combine_to_one(*w))
        //     .map(|w| w.to_lowercase())
        //     .collect::<Vec<String>>()
        //     .join("_")
    }

    fn parse_snake(input: &str) -> String {
        todo!()
    }

    fn parse_title_snake(input: &str) -> String {
        todo!()
    }

    fn parse_screaming_snake(input: &str) -> String {
        todo!()
    }

    fn parse_lower(input: &str) -> String {
        todo!()
    }
}

fn word_starting_with_upper(input: &str) -> IResult<&str, &str> {
    let (rest, (u, l)) = pair(take_till(is_uppercase), take_while(is_lowercase)).parse(input).unwrap();
    let w = u.to_string() + l;

    Ok((rest, w.as_str()))
}

fn combine_to_one<'a>(input: (&'a str, &'a str)) -> String {
    format!("{}{}", input.0, input.1)
}

fn is_uppercase(input: char) -> bool {
    input.is_uppercase()
}

fn is_lowercase(input: char) -> bool {
    input.is_lowercase()
}

fn get_upper_case_chars(input: &str) -> Vec<(usize, char)> {
    input
        .char_indices()
        .filter(|(_, c)| c.is_uppercase())
        .collect()
}

fn get_lower_case_chars(input: &str) -> Vec<(usize, char)> {
    input
        .char_indices()
        .filter(|(_, c)| c.is_lowercase())
        .collect()
}

fn find_casing(input: &str) -> Result<Casing, CasingParseError> {
    let regexes: HashMap<&str, Casing> = HashMap::from([
        (r#"^([A-Z][a-z]+)+$"#, Casing::PascalCase),
        (r#"([a-z]+_[a-z]+)+"#, Casing::SnakeCase),
        (r#"(\p{Lu}\p{Ll}+_\p{Lu}\p{Ll}+)+"#, Casing::TitleSnakeCase),
        (r#"(\p{Lu}+_\p{Lu}+)+"#, Casing::ScreamingSnakeCase),
        (r#"^(\p{Ll}+)$"#, Casing::LowerCase),
    ]);

    for (r, c) in regexes {
        let compiled_regex = Regex::new(r).unwrap();

        if compiled_regex.is_match(input) {
            return Ok(c);
        }
    }

    Err(CasingParseError::new(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_pascal_to_lower_snake() {
        let input = "ExampleValue";
        let expected = "example_value";

        let actual = parse(input);

        assert_eq!(expected, actual)
    }
}
