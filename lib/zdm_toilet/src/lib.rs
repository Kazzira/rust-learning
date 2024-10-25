pub mod zdm_toilet {
    use regex::Regex;

    /// This is the error code returned by Err in do_toiletify_word.
    #[derive(Debug, PartialEq)]
    pub enum ToiletErrorCode {
        /// This error code is returned when the word has a space.
        WordHasSpace,
        /// This error code is returned when the word is not transformed.
        NonToiletWord,
        /// This error code is returned when the regex returns an error.
        InternalError,
    }

    fn do_toiletify_word(word: &String) -> Option<String> {
        let re_result = Regex::new(r"[Tt][^Tt]+[Ll][^Tt]+[Tt]");
        let re: Regex;

        match re_result {
            Ok(r_re) => {
                re = r_re;
            }
            Err(_r_error) => {
                // I guess return none. Don't leak a regex error I guess...
                // Could have InternalError become RegexError and attach
                // the regex::Error type...
                return None;
            }
        }

        let new_word = re.replace(word.as_str(), "toilet").into_owned();

        Some(new_word)
    }

    /// Transforms a substring of a word into toilet based on certain conditons.
    ///
    /// This string or substring must begin with 't', have some letters (not t!)
    /// then 'l', then some letters(not t!), then 't'.
    ///
    /// Replace that substring with toilet.
    ///
    /// # Examples:
    /// - twilight => toilet
    /// - totalitarian => totoiletarian
    /// - teletypewriter => toiletypewriter
    ///
    /// # Arguments
    ///
    /// * 'word' - The word with no spaces.
    ///
    /// # Returns
    /// - String transformed if word meets the conditions above.
    /// - ToiletErrorCode::WordHasSpace if the word contains a space.
    /// - ToiletErrorCode::NonToiletWord if the word does not meet the conditions.
    /// - ToiletErrorCode::InternalError if the regex fails for some reason.
    ///
    pub fn toiletify_word(word: &String) -> Result<String, ToiletErrorCode> {
        // No words with spaces!
        if word.find(' ').is_some() {
            return Err(ToiletErrorCode::WordHasSpace);
        }

        let new_word_or_none = do_toiletify_word(&word);
        let new_word: String;

        match new_word_or_none {
            Some(r_new_word) => new_word = r_new_word,
            None => {
                return Err(ToiletErrorCode::InternalError);
            }
        }

        if new_word == *word {
            Err(ToiletErrorCode::NonToiletWord)
        } else {
            Ok(new_word)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::zdm_toilet;

    #[test]
    fn word_with_spaces_should_result_in_error() {
        let input: String = "Fun Times".to_owned();

        let toilet_result = zdm_toilet::toiletify_word(&input);

        match toilet_result {
            Ok(_new_word) => panic!("String has space and should returned and error"),
            Err(error_code) => {
                assert_eq!(error_code, zdm_toilet::ToiletErrorCode::WordHasSpace)
            }
        }
    }

    #[test]
    fn word_that_wont_apply_results_in_error() {
        let input: String = "Lahabrea".to_owned();

        let toilet_result = zdm_toilet::toiletify_word(&input);

        match toilet_result {
            Ok(_new_word) => panic!("String result should not apply!"),
            Err(error_code) => {
                assert_eq!(error_code, zdm_toilet::ToiletErrorCode::NonToiletWord)
            }
        }
    }

    #[test]
    fn test_twilight_becomes_toilet() {
        let input: String = "twilight".to_owned();

        // Can I match the function call?
        match zdm_toilet::toiletify_word(&input) {
            Ok(new_word) => assert_eq!(new_word, "toilet"),
            Err(_err) => {
                panic!("Should not result in error!")
            }
        }
    }

    #[test]
    fn test_totalitarian_becomes_totoiletarian() {
        let input: String = "totalitarian".to_owned();

        // Can I match the function call?
        match zdm_toilet::toiletify_word(&input) {
            Ok(new_word) => assert_eq!(new_word, "totoiletarian"),
            Err(_err) => {
                panic!("Should not result in error!")
            }
        }
    }
}
