<div align="center">

# 🇮🇷 pizza-analysis-persian

**Persian (Farsi) text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--persian-blue)](https://github.com/pizza-rs/analysis-persian)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Persian/Farsi language analysis with Arabic script normalization, Persian-specific
normalization, stemming, and stop words. Handles both Arabic and Persian character
variants since Persian uses a modified Arabic script.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `arabic_normalization` | Normalize shared Arabic script characters |
| TokenFilter | `persian_normalization` | Persian-specific char normalization (ک←ك, ی←ي) |
| TokenFilter | `persian_stem` | Persian light stemmer |
| TokenFilter | `persian_stop` | Persian stop words (308 entries) |
| Analyzer | `persian` | Full pipeline: lowercase → arabic_norm → persian_norm → stem → stop |

### Persian Normalization

| Input | Output | Rule |
|:------|:-------|:-----|
| ك (Arabic Kaf) | ک (Persian Kaf) | Script variant |
| ي (Arabic Ya) | ی (Persian Ya) | Script variant |
| ۀ (Ha + Hamza) | هٔ | Canonical form |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_persian::register_all(&mut factory);

let analyzer = factory.get_analyzer("persian").unwrap();
```

## Installation

```toml
[dependencies]
pizza-analysis-persian = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["persian"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
