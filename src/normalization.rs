use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Persian normalization filter.
///
/// Normalizes:
/// - Arabic Yeh (ي U+064A) → Persian Yeh (ی U+06CC)
/// - Arabic Keh (ك U+0643) → Persian Keh (ک U+06A9)
/// - Zero-width non-joiner (U+200C) → space (handled at tokenizer level)
/// - Arabic-Indic digits (٠-٩) → ASCII digits (0-9)
/// - Extended Arabic-Indic digits (۰-۹) → ASCII digits
#[derive(Clone, Debug, Default)]
pub struct PersianNormalizationFilter;

impl PersianNormalizationFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for PersianNormalizationFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let mut result = String::with_capacity(text.len());
        let mut changed = false;

        for c in text.chars() {
            let normalized = match c {
                // Arabic Yeh → Persian Yeh
                '\u{064A}' => { changed = true; '\u{06CC}' }
                // Arabic Keh → Persian Keh
                '\u{0643}' => { changed = true; '\u{06A9}' }
                // Arabic-Indic digits → ASCII
                '\u{0660}'..='\u{0669}' => {
                    changed = true;
                    char::from_u32(c as u32 - 0x0660 + '0' as u32).unwrap_or(c)
                }
                // Extended Arabic-Indic digits → ASCII
                '\u{06F0}'..='\u{06F9}' => {
                    changed = true;
                    char::from_u32(c as u32 - 0x06F0 + '0' as u32).unwrap_or(c)
                }
                // HEH with YEH → HEH
                '\u{06C0}' => { changed = true; '\u{06C1}' }
                // Various forms of Alef → plain Alef
                '\u{0622}' | '\u{0623}' | '\u{0625}' => { changed = true; '\u{0627}' }
                _ => c,
            };
            result.push(normalized);
        }

        if changed {
            token.term = Cow::Owned(result);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_yeh_to_persian() {
        let filter = PersianNormalizationFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("\u{064A}\u{064A}"), // Arabic Yeh
            start_offset: 0,
            end_offset: 4,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "\u{06CC}\u{06CC}");
    }

    #[test]
    fn test_digit_normalization() {
        let filter = PersianNormalizationFilter::new();
        let mut token = Token {
            term: Cow::Borrowed("۱۲۳"),
            start_offset: 0,
            end_offset: 6,
            position: 0,
        };
        let (deleted, _) = filter.filter(&mut token);
        assert!(!deleted);
        assert_eq!(token.term.as_ref(), "123");
    }
}
