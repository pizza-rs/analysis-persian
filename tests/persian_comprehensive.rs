//! Comprehensive tests for pizza-analysis-persian.

use pizza_analysis_persian::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// ArabicNormalizationFilter (reexported for Persian)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn arabic_normalization_construction() {
    let _f = ArabicNormalizationFilter::new();
}

#[test]
fn arabic_normalization_alef_forms() {
    let f = ArabicNormalizationFilter::new();
    let mut token = make_token("أحمد");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn arabic_normalization_diacritics() {
    let f = ArabicNormalizationFilter::new();
    let mut token = make_token("كَتَبَ");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('\u{064E}'));
}

#[test]
fn arabic_normalization_empty() {
    let f = ArabicNormalizationFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PersianNormalizationFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn persian_normalization_construction() {
    let _f = PersianNormalizationFilter::new();
}

#[test]
fn persian_normalization_yeh() {
    let f = PersianNormalizationFilter::new();
    // Arabic yeh (ي) → Persian yeh (ی)
    let mut token = make_token("عربي");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn persian_normalization_kaf() {
    let f = PersianNormalizationFilter::new();
    // Arabic kaf (ك) → Persian kaf (ک)
    let mut token = make_token("كتاب");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn persian_normalization_ascii_passthrough() {
    let f = PersianNormalizationFilter::new();
    let mut token = make_token("hello");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "hello");
}

#[test]
fn persian_normalization_empty() {
    let f = PersianNormalizationFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PersianStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = PersianStemFilter::new();
}

#[test]
fn stem_plural_ha() {
    let f = PersianStemFilter::new();
    // "کتابها" (books) → strip ها
    let mut token = make_token("کتابها");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_plural_an() {
    let f = PersianStemFilter::new();
    // "مردان" (men) → strip ان
    let mut token = make_token("مردان");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_suffix() {
    let f = PersianStemFilter::new();
    let mut token = make_token("نوشتم");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = PersianStemFilter::new();
    let mut token = make_token("در");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = PersianStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// PersianStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = PersianStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = PersianStopFilter::new();
    let stop_words = ["و", "در", "به", "از", "را", "با", "است", "اما", "ام", "اش"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = PersianStopFilter::new();
    let content_words = ["کتاب", "خانه", "مدرسه", "شهر"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = PersianStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("arabic_normalization").is_some());
    assert!(factory.get_token_filter("persian_normalization").is_some());
    assert!(factory.get_token_filter("persian_stem").is_some());
    assert!(factory.get_token_filter("persian_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("persian").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("persian").unwrap();
    let mut input = String::from("کتاب در خانه است");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("persian").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("persian").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
