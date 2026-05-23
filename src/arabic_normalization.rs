//! Arabic normalization filter — applies Arabic script normalizations needed
//! before Persian-specific normalization.
//!
//! Equivalent to Lucene's `ArabicNormalizationFilter`. Normalizes:
//! - Removes tatweel (kashida, U+0640)
//! - Removes Arabic diacritics (tashkeel): fathatan, dammatan, kasratan,
//!   fatha, damma, kasra, shadda, sukun, superscript alef
//! - Dotless Yeh (U+0649) → Yeh (U+064A)
//! - Teh Marbuta (U+0629) → Heh (U+0647)

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Removes Arabic diacritics, tatweel, and normalizes some letter forms.
///
/// This filter should be applied *before* Persian normalization so that
/// diacritics don't interfere with downstream processing.
#[derive(Clone, Debug, Default)]
pub struct ArabicNormalizationFilter;

impl ArabicNormalizationFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for ArabicNormalizationFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.is_empty() {
            return (false, None);
        }

        let mut result = String::with_capacity(text.len());
        let mut changed = false;

        for c in text.chars() {
            match c {
                // Tatweel (kashida) — remove
                '\u{0640}' => { changed = true; }
                // Arabic diacritics (tashkeel) — remove
                '\u{064B}' // FATHATAN
                | '\u{064C}' // DAMMATAN
                | '\u{064D}' // KASRATAN
                | '\u{064E}' // FATHA
                | '\u{064F}' // DAMMA
                | '\u{0650}' // KASRA
                | '\u{0651}' // SHADDA
                | '\u{0652}' // SUKUN
                | '\u{0670}' // SUPERSCRIPT ALEF
                => { changed = true; }
                // Dotless Yeh (ALEF MAKSURA) → Yeh
                '\u{0649}' => { result.push('\u{064A}'); changed = true; }
                // Teh Marbuta → Heh
                '\u{0629}' => { result.push('\u{0647}'); changed = true; }
                _ => { result.push(c); }
            }
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
    fn test_remove_tatweel() {
        let f = ArabicNormalizationFilter::new();
        let mut token = Token::new("كـــتاب", 0, 14, 0); // kitāb with tatweel
        f.filter(&mut token);
        assert_eq!(token.term, "كتاب");
    }

    #[test]
    fn test_remove_diacritics() {
        let f = ArabicNormalizationFilter::new();
        // "kitaabun" with fatha + kasra + fathatan
        let mut token = Token::new("كِتَابٌ", 0, 14, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "كتاب");
    }

    #[test]
    fn test_dotless_yeh() {
        let f = ArabicNormalizationFilter::new();
        let mut token = Token::new("مبنى", 0, 8, 0); // ends with ALEF MAKSURA
        f.filter(&mut token);
        assert_eq!(token.term, "مبني"); // ends with YEH
    }

    #[test]
    fn test_teh_marbuta() {
        let f = ArabicNormalizationFilter::new();
        let mut token = Token::new("مدرسة", 0, 10, 0); // ends with TEH MARBUTA
        f.filter(&mut token);
        assert_eq!(token.term, "مدرسه"); // ends with HEH
    }

    #[test]
    fn test_no_change() {
        let f = ArabicNormalizationFilter::new();
        let mut token = Token::new("فارسی", 0, 10, 0);
        let original = token.term.as_ref().to_string();
        f.filter(&mut token);
        assert_eq!(token.term.as_ref(), original);
    }
}
