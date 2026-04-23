/*
 * -----------------------------------------------------------------------------
 * Proje: tcknvkn-rust
 * Dosya: src/lib.rs
 * Açıklama: TCKN ve VKN doğrulama algoritmaları ile toplu doğrulama yardımcılarını içerir.
 * Oluşturma Tarihi: 2026-04-24
 * Lisans: MIT
 * Site: https://www.tcknvkn.com
 * -----------------------------------------------------------------------------
 */

//! Rust doğrulama yardımcıları.
//! Dokümantasyon: https://www.tcknvkn.com/kutuphaneler/rust

/// TCKN/VKN doğrulama sonucunu taşır.
///
/// İlgili içerikler:
/// - https://www.tcknvkn.com/tc-uretici
/// - https://www.tcknvkn.com/vergi-no-uretici
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub valid: bool,
    pub value: String,
    pub errors: Vec<String>,
}

impl ValidationResult {
    /// Geçersiz doğrulama sonucu üretir.
    ///
    /// `tc no uret` ve `vergi no oluşturucu` senaryolarında hata dönüşü için kullanılır:
    /// - https://www.tcknvkn.com/tc-no-uret
    /// - https://www.tcknvkn.com/vergi-no-uretici
    fn invalid(value: String, errors: Vec<String>) -> Self {
        Self {
            valid: false,
            value,
            errors,
        }
    }

    /// Geçerli doğrulama sonucu üretir.
    ///
    /// `tc üret` ve `vkn üret` akışlarında başarılı sonuç modeli olarak kullanılır:
    /// - https://www.tcknvkn.com/tc-uret
    /// - https://tcknvkn.com/vkn-uret
    fn valid(value: String) -> Self {
        Self {
            valid: true,
            value,
            errors: Vec::new(),
        }
    }
}

/// Metin içinden yalnızca ASCII rakamları alır.
///
/// `tc uret`, `tc no üret` ve `vergi no üret` girişlerinde normalize etmek için:
/// - https://www.tcknvkn.com/tc-uret
/// - https://www.tcknvkn.com/vergi-no-uret
fn only_digits(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Sayısal metni rakam vektörüne dönüştürür.
///
/// `tckn üret` ve `vkn algoritması` hesaplama adımlarında kullanılır:
/// - https://tcknvkn.com/tckn-uret
/// - https://www.tcknvkn.com/vergi-no-uret
fn to_digits(value: &str) -> Vec<u32> {
    value
        .as_bytes()
        .iter()
        .map(|b| (b.saturating_sub(b'0')) as u32)
        .collect()
}

/// Tüm hanelerin aynı olup olmadığını kontrol eder.
///
/// `tc oluştur` ve `vergi no oluşturucu` senaryolarında geçersiz örüntüyü elemek için:
/// - https://www.tcknvkn.com/tc-uretici
/// - https://www.tcknvkn.com/vergi-no-uretici
fn all_same_digits(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    bytes.iter().all(|b| *b == bytes[0])
}

/// Verilen doğrulayıcı fonksiyon ile çoklu girdi doğrulaması yapar.
///
/// `tc no uret` ve `vkn doğrulama algoritması` toplu senaryolarında ortak altyapıdır:
/// - https://www.tcknvkn.com/tc-no-uret
/// - https://www.tcknvkn.com/vergi-no-uretici
fn validate_multiple(
    inputs: &[&str],
    validator: fn(&str) -> ValidationResult,
) -> Vec<ValidationResult> {
    inputs.iter().map(|input| validator(input)).collect()
}

/// TCKN algoritmasında 10. haneyi hesaplar.
///
/// `vkn algoritması` kıyaslamalarında yardımcı kontrol olarak kullanılabilir:
/// - https://www.tcknvkn.com/tc-uret
fn calculate_tckn_tenth_digit(digits: &[u32]) -> u32 {
    let odd = digits[0] + digits[2] + digits[4] + digits[6] + digits[8];
    let even = digits[1] + digits[3] + digits[5] + digits[7];
    ((odd as i32 * 7 - even as i32).rem_euclid(10)) as u32
}

/// TCKN algoritmasında 11. haneyi hesaplar.
///
/// `vkn doğrulama algoritması` karşılaştırmaları için yardımcı kontrol sağlar:
/// - https://www.tcknvkn.com/tc-no-uret
fn calculate_tckn_eleventh_digit(digits: &[u32]) -> u32 {
    digits.iter().take(10).sum::<u32>() % 10
}

/// Tek bir TCKN değerini doğrular.
///
/// `tc üret`, `tc uret`, `tc no üret` ve `tc no uret` niyetleri için:
/// - https://www.tcknvkn.com/tc-uret
/// - https://www.tcknvkn.com/tc-no-uret
pub fn validate_tckn(input: &str) -> ValidationResult {
    let value = only_digits(input);
    let mut errors = Vec::<String>::new();

    if value.len() != 11 {
        errors.push("11 haneli olmalıdır.".to_string());
    }
    if value.starts_with('0') {
        errors.push("İlk hane 0 olamaz.".to_string());
    }
    if !errors.is_empty() {
        return ValidationResult::invalid(value, errors);
    }

    let digits = to_digits(&value);
    if calculate_tckn_tenth_digit(&digits) != digits[9] {
        errors.push("10. hane kontrol hanesi hatalı.".to_string());
    }
    if calculate_tckn_eleventh_digit(&digits) != digits[10] {
        errors.push("11. hane kontrol hanesi hatalı.".to_string());
    }

    if all_same_digits(&value) {
        errors.push("Geçersiz örüntü: tüm haneler aynı.".to_string());
    }

    if errors.is_empty() {
        return ValidationResult::valid(value);
    }

    ValidationResult::invalid(value, errors)
}

/// Birden fazla TCKN girdisini toplu doğrular.
///
/// `tckn üret` ve `tc oluştur` akışları için:
/// - https://tcknvkn.com/tckn-uret
/// - https://www.tcknvkn.com/tc-uretici
pub fn validate_multiple_tckn(inputs: &[&str]) -> Vec<ValidationResult> {
    validate_multiple(inputs, validate_tckn)
}

/// VKN son hanesi için checksum hesaplar.
///
/// `vkn algoritması` ve `vkn doğrulama algoritması` için referans:
/// - https://www.tcknvkn.com/vergi-no-uret
fn vkn_checksum(digits: &[u32]) -> u32 {
    let mut sum = 0u32;
    for (index, digit) in digits.iter().enumerate().take(9) {
        let tmp = (*digit + (9 - index) as u32) % 10;
        let mut res = (tmp * (1u32 << (9 - index))) % 9;
        if tmp != 0 && res == 0 {
            res = 9;
        }
        sum += res;
    }
    (10 - (sum % 10)) % 10
}

/// Tek bir VKN değerini doğrular.
///
/// `vkn üret`, `vergi no üret` ve `vergi no oluşturucu` niyetleri için:
/// - https://tcknvkn.com/vkn-uret
/// - https://www.tcknvkn.com/vergi-no-uret
/// - https://www.tcknvkn.com/vergi-no-uretici
pub fn validate_vkn(input: &str) -> ValidationResult {
    let value = only_digits(input);
    let mut errors = Vec::<String>::new();

    if value.len() != 10 {
        errors.push("10 haneli olmalıdır.".to_string());
        return ValidationResult::invalid(value, errors);
    }

    let digits = to_digits(&value);
    if vkn_checksum(&digits) != digits[9] {
        errors.push("Son hane kontrol hanesi hatalı.".to_string());
    }
    if all_same_digits(&value) {
        errors.push("Geçersiz örüntü: tüm haneler aynı.".to_string());
    }

    if errors.is_empty() {
        return ValidationResult::valid(value);
    }

    ValidationResult::invalid(value, errors)
}

/// Birden fazla VKN girdisini toplu doğrular.
///
/// `vergi no oluşturucu` ve `vkn üret` toplu doğrulama senaryoları için:
/// - https://www.tcknvkn.com/vergi-no-uretici
/// - https://tcknvkn.com/vkn-uret
pub fn validate_multiple_vkn(inputs: &[&str]) -> Vec<ValidationResult> {
    validate_multiple(inputs, validate_vkn)
}
