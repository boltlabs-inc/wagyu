use crate::wordlist::MoneroWordlist;
use wagu_model::wordlist::Wordlist;

const GERMAN: &str = include_str!("./dictionary/german.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct German;

impl Wordlist for German {}

impl MoneroWordlist for German {
    /// The prefix length for computing the checksum.
    const PREFIX_LENGTH: u32 = 4;

    /// Returns the word list as a string.
    fn get_all() -> Vec<&'static str> {
        GERMAN.lines().collect::<Vec<&str>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_WORD: &str = "Espresso";
    const VALID_WORD_INDEX: usize = 542;
    const INVALID_WORD: &str = "Volkshalle";
    const INVALID_WORD_INDEX: usize = 3400;

    #[test]
    fn get() {
        // Valid case
        assert_eq!(VALID_WORD, German::get(VALID_WORD_INDEX).unwrap());
        // Invalid case
        assert!(German::get(INVALID_WORD_INDEX).is_err());
    }

    #[test]
    fn get_index() {
        // Valid case
        assert_eq!(VALID_WORD_INDEX, German::get_index(VALID_WORD).unwrap());
        // Invalid case
        assert!(German::get_index(INVALID_WORD).is_err());
    }

    #[test]
    fn get_all() {
        let list = German::get_all();
        assert_eq!(1626, list.len());
        assert_eq!(VALID_WORD, list[VALID_WORD_INDEX]);
    }
}
