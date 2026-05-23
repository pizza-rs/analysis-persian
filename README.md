# pizza-analysis-persian

Persian language analysis with Arabic normalization layer, Persian-specific normalization, stemming, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `arabic_normalization` | Token Filter | Removes tatweel, Arabic diacritics; normalizes dotless Yeh and Teh Marbuta |
| `persian_normalization` | Token Filter | Persian-specific: Yeh/Keh normalization, digit conversion, Alef forms |
| `persian_stem` | Token Filter | Persian light stemmer |
| `persian_stop` | Token Filter | Persian stop words filter (308 words) |
| `persian` | Analyzer | Full pipeline: arabic_normalization → persian_normalization → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "persian"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["arabic_normalization", "persian_normalization", "persian_stem", "persian_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
