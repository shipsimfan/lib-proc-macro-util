//! The set of languages supported by this library

use i18n_translation::locale::LanguageTag;

/// English (United States)
pub const EN: &LanguageTag = &LanguageTag::from_language(b"en").unwrap();

/// French (France)
pub const FR: &LanguageTag = &LanguageTag::from_language(b"fr").unwrap();

/// Chinese (Simplified, mainland China)
pub const ZH: &LanguageTag = &LanguageTag::from_language(b"zh").unwrap();
