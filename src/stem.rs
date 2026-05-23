use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Persian stemmer that removes common suffixes.
/// Handles plural markers, indefinite articles, and possessive enclitics.
#[derive(Clone, Debug, Default)]
pub struct PersianStemFilter;

impl PersianStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for PersianStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let char_count = text.chars().count();
        if char_count < 4 {
            return (false, None);
        }

        let stemmed = stem_persian(text, char_count);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_persian(word: &str, len: usize) -> String {
    let chars: Vec<char> = word.chars().collect();

    // Longest suffixes first (3+ chars)
    if len > 5 {
        let suffix3: String = chars[len - 3..].iter().collect();
        match suffix3.as_str() {
            "ها\u{06CC}" | "های" => return chars[..len - 3].iter().collect(), // plural + ezafe
            "ات\u{06CC}" | "اتی" => return chars[..len - 3].iter().collect(),
            _ => {}
        }
    }

    // 2-char suffixes
    if len > 4 {
        let suffix2: String = chars[len - 2..].iter().collect();
        match suffix2.as_str() {
            "ها" => return chars[..len - 2].iter().collect(), // plural -hā
            "ان" => return chars[..len - 2].iter().collect(), // plural -ān
            "ات" => return chars[..len - 2].iter().collect(), // plural -āt (Arabic)
            "تر" => return chars[..len - 2].iter().collect(), // comparative
            "ای" => return chars[..len - 2].iter().collect(), // indefinite -i (ezafe)
            "اش" => return chars[..len - 2].iter().collect(), // possessive
            "ام" => return chars[..len - 2].iter().collect(), // 1st person
            "شان" => return chars[..len - 2].iter().collect(), // 3rd pl possessive
            _ => {}
        }
    }

    // 1-char suffixes
    if len > 3 {
        let last = chars[len - 1];
        match last {
            '\u{06CC}' | 'ی' => return chars[..len - 1].iter().collect(), // indefinite -i
            _ => {}
        }
    }

    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stem_plural_ha() {
        let filter = PersianStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("کتابها"),
            start_offset: 0,
            end_offset: 12,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "کتاب");
    }

    #[test]
    fn test_short_word_unchanged() {
        let filter = PersianStemFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("من"),
            start_offset: 0,
            end_offset: 4,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "من");
    }
}
