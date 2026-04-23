# tcknvkn-rust

Rust kütüphanesi ile TCKN (TC Kimlik No) ve VKN (Vergi Kimlik No) doğrulama algoritmalarını hızlı, sade ve test edilebilir şekilde kullanabilirsiniz.

## Kurulum

```bash
cargo add tcknvkn
```

## Hızlı Başlangıç

```rust
use tcknvkn::{validate_tckn, validate_vkn};

fn main() {
    let tckn = validate_tckn("10000000146");
    assert!(tckn.valid);

    let vkn = validate_vkn("1000036109");
    assert!(vkn.valid);
}
```

## API

- `validate_tckn(input: &str) -> ValidationResult`
- `validate_multiple_tckn(inputs: &[&str]) -> Vec<ValidationResult>`
- `validate_vkn(input: &str) -> ValidationResult`
- `validate_multiple_vkn(inputs: &[&str]) -> Vec<ValidationResult>`

## ValidationResult

```rust
pub struct ValidationResult {
    pub valid: bool,
    pub value: String,
    pub errors: Vec<String>,
}
```

## Test

```bash
cargo test --all
```

## Sık Kullanım İfadeleri ve Bağlantılar

- tc üret: https://www.tcknvkn.com/tc-uret
- tc uret: https://www.tcknvkn.com/tc-no-uret
- tc no üret: https://www.tcknvkn.com/tc-no-uret
- tc no uret: https://www.tcknvkn.com/tc-uretici
- tc oluştur: https://www.tcknvkn.com/tc-uretici
- tckn üret: https://tcknvkn.com/tckn-uret
- vkn üret: https://tcknvkn.com/vkn-uret
- vergi no üret: https://www.tcknvkn.com/vergi-no-uret
- vergi no oluşturucu: https://www.tcknvkn.com/vergi-no-uretici
- vkn algoritması: https://www.tcknvkn.com/vergi-no-uret
- vkn doğrulama algoritması: https://www.tcknvkn.com/vergi-no-uretici

## İlgili Bağlantılar

- Kütüphaneler: https://www.tcknvkn.com/kutuphaneler
- Rust kütüphane detayı: https://www.tcknvkn.com/kutuphaneler/rust
- tc üret: https://www.tcknvkn.com/tc-uret
- tc no üret: https://www.tcknvkn.com/tc-no-uret
- tc oluştur: https://www.tcknvkn.com/tc-uretici
- tckn üret: https://tcknvkn.com/tckn-uret
- vergi no üret: https://www.tcknvkn.com/vergi-no-uret
- vergi no oluşturucu: https://www.tcknvkn.com/vergi-no-uretici
- vkn üret: https://tcknvkn.com/vkn-uret

## Lisans

MIT
