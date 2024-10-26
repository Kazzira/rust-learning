pub mod zdm_toilet {
    use regex::Regex;

    /// This is the error code returned by Err in do_toiletify_word.
    #[derive(Debug, PartialEq)]
    pub enum Error {
        /// This error code is returned when the word has a space.
        WordHasSpace,
        /// This error code is returned when the word is not transformed.
        NonToiletWord,
        /// This error code is returned when the regex returns an error.
        InternalRegexError(regex::Error),
    }

    fn do_toiletify_word(word: &str) -> Result<String, regex::Error> {
        let re_result = Regex::new(r"[Tt][^Tt]+[Ll][^Tt]+[Tt]");
        let re: Regex;

        match re_result {
            Ok(r_re) => {
                re = r_re;
            }
            Err(r_error) => {
                return Err(r_error);
            }
        }

        let new_word = re.replace(word, "toilet").into_owned();
        Ok(new_word)
    }

    /// Transforms a substring of a word into toilet based on certain conditons.
    ///
    /// This string or substring must begin with 't', have some letters (not t!)
    /// then 'l', then some letters(not t!), then 't'.
    ///
    /// Replace that substring with toilet.
    ///
    /// # Examples
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
    /// - Error::WordHasSpace if the word contains a space.
    /// - Error::NonToiletWord if the word does not meet the conditions.
    /// - Error::InternalRegexError if the regex fails for some reason.
    ///
    pub fn toiletify_word(word: &str) -> Result<String, Error> {
        // No words with spaces!
        if word.find(' ').is_some() {
            return Err(Error::WordHasSpace);
        }

        let new_word_or_none = do_toiletify_word(&word);
        let new_word: String;

        match new_word_or_none {
            Ok(r_new_word) => new_word = r_new_word,
            Err(r_error) => {
                return Err(Error::InternalRegexError(r_error));
            }
        }

        if new_word == *word {
            Err(Error::NonToiletWord)
        } else {
            Ok(new_word)
        }
    }

    #[test]
    fn word_with_spaces_should_result_in_error() {
        let input: String = "Fun Times".to_owned();

        let toilet_result = toiletify_word(&input);

        match toilet_result {
            Ok(_new_word) => panic!("String has space and should returned and error"),
            Err(error_code) => {
                assert_eq!(error_code, Error::WordHasSpace)
            }
        }
    }

    #[test]
    fn word_that_wont_apply_results_in_error() {
        let input: String = "Lahabrea".to_owned();

        let toilet_result = toiletify_word(&input);

        match toilet_result {
            Ok(_new_word) => panic!("String result should not apply!"),
            Err(error_code) => {
                assert_eq!(error_code, Error::NonToiletWord)
            }
        }
    }

    #[test]
    fn test_twilight_becomes_toilet() {
        let input: String = "twilight".to_owned();

        // Can I match the function call?
        match toiletify_word(&input) {
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
        match toiletify_word(&input) {
            Ok(new_word) => assert_eq!(new_word, "totoiletarian"),
            Err(_err) => {
                panic!("Should not result in error!")
            }
        }
    }
}
