//! Persian stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::{Token, TokenFilter};

/// Default Persian stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
    "اباد",
    "اثر",
    "اره",
    "اري",
    "از",
    "است",
    "استفاده",
    "اش",
    "اكنون",
    "البته",
    "ام",
    "اما",
    "امد",
    "امده",
    "امروز",
    "امسال",
    "ان",
    "انان",
    "انجا",
    "اند",
    "انكه",
    "انها",
    "انچه",
    "او",
    "اورد",
    "اورده",
    "اول",
    "اي",
    "ايا",
    "ايد",
    "ايشان",
    "ايم",
    "اين",
    "اينكه",
    "اگر",
    "با",
    "بار",
    "باره",
    "باشد",
    "باشند",
    "باشيم",
    "بالا",
    "بالاي",
    "بايد",
    "بدون",
    "بر",
    "برابر",
    "براساس",
    "براي",
    "برخوردار",
    "برخي",
    "برداري",
    "بروز",
    "بسيار",
    "بسياري",
    "بعد",
    "بعري",
    "بعضي",
    "بلكه",
    "بله",
    "بلي",
    "بنابراين",
    "بندي",
    "به",
    "بهترين",
    "بود",
    "بودن",
    "بودند",
    "بوده",
    "بي",
    "بيرون",
    "بيست",
    "بيش",
    "بيشتر",
    "بيشتري",
    "بين",
    "تا",
    "تازه",
    "تاكنون",
    "تان",
    "تحت",
    "تر",
    "ترين",
    "تمام",
    "تمامي",
    "تنها",
    "تواند",
    "توانند",
    "توسط",
    "تول",
    "توي",
    "جا",
    "جاي",
    "جايي",
    "جدا",
    "جديد",
    "جريان",
    "جز",
    "جلوي",
    "جلوگيري",
    "حتي",
    "حدود",
    "حق",
    "خارج",
    "خدمات",
    "خواست",
    "خواهد",
    "خواهند",
    "خواهيم",
    "خود",
    "خويش",
    "خياه",
    "داد",
    "دادن",
    "دادند",
    "داده",
    "دارد",
    "دارند",
    "داريم",
    "داشت",
    "داشتن",
    "داشتند",
    "داشته",
    "دانست",
    "دانند",
    "در",
    "درباره",
    "دنبال",
    "ده",
    "دهد",
    "دهند",
    "دو",
    "دوم",
    "ديده",
    "ديروز",
    "ديگر",
    "ديگران",
    "ديگري",
    "را",
    "راه",
    "رفت",
    "رفته",
    "روب",
    "روزهاي",
    "روي",
    "ريزي",
    "زياد",
    "زير",
    "زيرا",
    "سابق",
    "ساخته",
    "سازي",
    "سراسر",
    "سري",
    "سعي",
    "سمت",
    "سوم",
    "سوي",
    "سپس",
    "شان",
    "شايد",
    "شد",
    "شدن",
    "شدند",
    "شده",
    "شش",
    "شما",
    "شناسي",
    "شود",
    "شوند",
    "صورت",
    "ضد",
    "ضمن",
    "طبق",
    "طريق",
    "طور",
    "طي",
    "عقب",
    "علت",
    "عنوان",
    "غير",
    "فقط",
    "فكر",
    "فوق",
    "قابل",
    "قبل",
    "قصد",
    "كجا",
    "كجاست",
    "كدام",
    "كرد",
    "كردم",
    "كردن",
    "كردند",
    "كرده",
    "كس",
    "كسي",
    "كل",
    "كمتر",
    "كنار",
    "كند",
    "كنم",
    "كنند",
    "كنيد",
    "كنيم",
    "كه",
    "كي",
    "لطفا",
    "ما",
    "مان",
    "مانند",
    "مثل",
    "مختلف",
    "مدتي",
    "مردم",
    "مرسي",
    "مقابل",
    "من",
    "مورد",
    "مي",
    "ميليارد",
    "ميليون",
    "مگر",
    "ناشي",
    "نام",
    "نبايد",
    "نبود",
    "نخست",
    "نخستين",
    "نخواهد",
    "ندارد",
    "ندارند",
    "نداشته",
    "نزد",
    "نزديك",
    "نشان",
    "نشده",
    "نظير",
    "نكرده",
    "نمايد",
    "نمي",
    "نه",
    "نوعي",
    "نيز",
    "نيست",
    "ه",
    "ها",
    "هاي",
    "هايي",
    "هر",
    "هرگز",
    "هزار",
    "هست",
    "هستند",
    "هستيم",
    "هفت",
    "هم",
    "همان",
    "همه",
    "همواره",
    "همين",
    "همچنان",
    "همچنين",
    "همچون",
    "هنوز",
    "هنگام",
    "هنگامي",
    "هيچ",
    "و",
    "وسط",
    "وقتي",
    "وقتيكه",
    "ولي",
    "وي",
    "وگو",
    "يا",
    "يابد",
    "يك",
    "يكديگر",
    "يكي",
    "پاعين",
    "پس",
    "پنج",
    "پيش",
    "چرا",
    "چطور",
    "چند",
    "چندين",
    "چنين",
    "چه",
    "چهار",
    "چون",
    "چيز",
    "چيزي",
    "چيست",
    "چگونه",
    "گذاري",
    "گذاشته",
    "گردد",
    "گرفت",
    "گرفته",
    "گروهي",
    "گفت",
    "گفته",
    "گويد",
    "گويند",
    "گيرد",
    "گيري",
    ];
    words.iter().copied().collect()
});

/// Removes Persian stop words from the token stream.
#[derive(Clone, Debug)]
pub struct PersianStopFilter {
    stop_words: HashSet<String>,
}

impl Default for PersianStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl PersianStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for PersianStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 308);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = PersianStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = PersianStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = PersianStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
