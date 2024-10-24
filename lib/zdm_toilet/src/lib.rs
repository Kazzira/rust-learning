pub mod zdm_toilet {
    use regex::Regex;

    #[derive(Debug, PartialEq)]
    pub enum ToiletErrorCode {
        WordHasSpace,
        NonToiletWord,
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
}
