use alloc::boxed::Box;
use alloc::vec;
use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;
use pizza_engine::analysis::StandardTokenizer;
use pizza_engine::analysis::TokenFilter;

use crate::arabic_normalization::ArabicNormalizationFilter;
use crate::normalization::PersianNormalizationFilter;
use crate::stem::PersianStemFilter;
use crate::stop::PersianStopFilter;

/// Register all Persian analysis components.
///
/// Registers:
/// - `"persian"` analyzer (arabic_normalization → persian_normalization → stop → stem)
/// - `"arabic_normalization"` token filter
/// - `"persian_normalization"` token filter
/// - `"persian_stem"` token filter
/// - `"persian_stop"` token filter
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("arabic_normalization", Box::new(ArabicNormalizationFilter::new()));
    factory.register_token_filter("persian_normalization", Box::new(PersianNormalizationFilter::new()));
    factory.register_token_filter("persian_stem", Box::new(PersianStemFilter::new()));
    factory.register_token_filter("persian_stop", Box::new(PersianStopFilter::new()));

    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(ArabicNormalizationFilter::new()),
        Box::new(PersianNormalizationFilter::new()),
        Box::new(PersianStopFilter::new()),
        Box::new(PersianStemFilter::new()),
    ];

    factory.register_analyzer(
        "persian",
        Analyzer::new(vec![], Box::new(StandardTokenizer::new()), filters),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("persian_normalization").is_some());
        assert!(factory.get_token_filter("persian_stem").is_some());
        assert!(factory.get_token_filter("persian_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("persian").is_some());
    }
}
