/*
 * -----------------------------------------------------------------------------
 * Proje: tcknvkn-rust
 * Dosya: tests/validation_tests.rs
 * Açıklama: TCKN ve VKN doğrulama akışları için varyasyonlu entegrasyon testlerini içerir.
 * Oluşturma Tarihi: 2026-04-24
 * Lisans: MIT
 * Site: https://www.tcknvkn.com
 * -----------------------------------------------------------------------------
 */

use tcknvkn::{
    validate_multiple_tckn, validate_multiple_vkn, validate_tckn, validate_vkn, ValidationResult,
};

/// Hata listesinin beklenen hata metnini içerdiğini doğrular.
fn contains_error(result: &ValidationResult, expected: &str) {
    assert!(
        result.errors.iter().any(|e| e == expected),
        "Beklenen hata '{}' bulunamadı. Hatalar: {:?}",
        expected,
        result.errors
    );
}

/// Geçerli bir TCKN değeri için doğrulama sonucunu test eder.
#[test]
fn validate_tckn_accepts_known_valid_value() {
    let result = validate_tckn("10000000146");
    assert!(result.valid);
    assert_eq!(result.value, "10000000146");
    assert!(result.errors.is_empty());
}

/// Rakam dışı karakter içeren TCKN girdisinin normalize edildiğini test eder.
#[test]
fn validate_tckn_normalizes_non_digit_chars() {
    let result = validate_tckn("100-000 00146");
    assert!(result.valid);
    assert_eq!(result.value, "10000000146");
}

/// Uzunluğu hatalı TCKN girdisinin reddedildiğini test eder.
#[test]
fn validate_tckn_rejects_invalid_length() {
    let result = validate_tckn("12345");
    assert!(!result.valid);
    contains_error(&result, "11 haneli olmalıdır.");
}

/// İlk hanesi 0 olan TCKN girdisinin reddedildiğini test eder.
#[test]
fn validate_tckn_rejects_leading_zero() {
    let result = validate_tckn("01234567890");
    assert!(!result.valid);
    contains_error(&result, "İlk hane 0 olamaz.");
}

/// Hatalı checksum içeren TCKN girdisinin reddedildiğini test eder.
#[test]
fn validate_tckn_rejects_checksum_mismatch() {
    let result = validate_tckn("10000000145");
    assert!(!result.valid);
    contains_error(&result, "11. hane kontrol hanesi hatalı.");
}

/// Tüm haneleri aynı olan TCKN girdisinin reddedildiğini test eder.
#[test]
fn validate_tckn_rejects_identical_digits() {
    let result = validate_tckn("11111111111");
    assert!(!result.valid);
    contains_error(&result, "Geçersiz örüntü: tüm haneler aynı.");
}

/// Kısa ve ilk hanesi 0 olan TCKN girdisinde birden fazla hata döndüğünü test eder.
#[test]
fn validate_tckn_collects_multiple_errors() {
    let result = validate_tckn("0");
    assert!(!result.valid);
    contains_error(&result, "11 haneli olmalıdır.");
    contains_error(&result, "İlk hane 0 olamaz.");
}

/// Çoklu TCKN doğrulamasında her girdi için sonuç üretildiğini test eder.
#[test]
fn validate_multiple_tckn_returns_result_for_each_input() {
    let results = validate_multiple_tckn(&["10000000146", "10000000145", "11111111111"]);
    assert_eq!(results.len(), 3);
    assert!(results[0].valid);
    assert!(!results[1].valid);
    assert!(!results[2].valid);
}

/// Boş TCKN listesinde boş sonuç döndüğünü test eder.
#[test]
fn validate_multiple_tckn_handles_empty_input() {
    let results = validate_multiple_tckn(&[]);
    assert!(results.is_empty());
}

/// Geçerli bir VKN değeri için doğrulama sonucunu test eder.
#[test]
fn validate_vkn_accepts_known_valid_value() {
    let result = validate_vkn("1000036109");
    assert!(result.valid);
    assert_eq!(result.value, "1000036109");
    assert!(result.errors.is_empty());
}

/// Rakam dışı karakter içeren VKN girdisinin normalize edildiğini test eder.
#[test]
fn validate_vkn_normalizes_non_digit_chars() {
    let result = validate_vkn("100-003-6109");
    assert!(result.valid);
    assert_eq!(result.value, "1000036109");
}

/// Uzunluğu hatalı VKN girdisinin reddedildiğini test eder.
#[test]
fn validate_vkn_rejects_invalid_length() {
    let result = validate_vkn("1234");
    assert!(!result.valid);
    contains_error(&result, "10 haneli olmalıdır.");
}

/// Hatalı checksum içeren VKN girdisinin reddedildiğini test eder.
#[test]
fn validate_vkn_rejects_checksum_mismatch() {
    let result = validate_vkn("1000036108");
    assert!(!result.valid);
    contains_error(&result, "Son hane kontrol hanesi hatalı.");
}

/// Tüm haneleri aynı olan VKN girdisinin reddedildiğini test eder.
#[test]
fn validate_vkn_rejects_identical_digits() {
    let result = validate_vkn("1111111111");
    assert!(!result.valid);
    contains_error(&result, "Geçersiz örüntü: tüm haneler aynı.");
}

/// Çoklu VKN doğrulamasında her girdi için sonuç üretildiğini test eder.
#[test]
fn validate_multiple_vkn_returns_result_for_each_input() {
    let results = validate_multiple_vkn(&["1000036109", "1000036108", "1111111111"]);
    assert_eq!(results.len(), 3);
    assert!(results[0].valid);
    assert!(!results[1].valid);
    assert!(!results[2].valid);
}

/// Boş VKN listesinde boş sonuç döndüğünü test eder.
#[test]
fn validate_multiple_vkn_handles_empty_input() {
    let results = validate_multiple_vkn(&[]);
    assert!(results.is_empty());
}
