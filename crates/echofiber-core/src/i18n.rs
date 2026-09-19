//! Localisation support for `EchoFiber` user-facing messages.
//!
//! Translations are embedded from `locales/<BCP-47 language tag>/messages.ftl`.

use std::fmt;

use fluent_bundle::{FluentBundle, FluentResource};
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;

const DEFAULT_LOCALE: &str = "en-US";
const RESOURCE_NAME: &str = "messages.ftl";

#[derive(RustEmbed)]
#[folder = "../../locales/"]
struct Locales;

/// An error while loading or formatting a translated message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalisationError {
    /// A selected translation resource could not be loaded or parsed.
    InvalidResource { locale: String },
    /// A requested message identifier is absent from the selected translation.
    MissingMessage { locale: String, message_id: String },
    /// A requested message has no value.
    MissingMessageValue { locale: String, message_id: String },
    /// Fluent could not format a message.
    Formatting { locale: String, message_id: String },
}

impl fmt::Display for LocalisationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidResource { locale } => {
                write!(formatter, "translation resource for locale `{locale}` is invalid")
            }
            Self::MissingMessage { locale, message_id } => {
                write!(formatter, "message `{message_id}` is missing from locale `{locale}`")
            }
            Self::MissingMessageValue { locale, message_id } => {
                write!(formatter, "message `{message_id}` has no value in locale `{locale}`")
            }
            Self::Formatting { locale, message_id } => {
                write!(
                    formatter,
                    "message `{message_id}` could not be formatted for locale `{locale}`"
                )
            }
        }
    }
}

impl std::error::Error for LocalisationError {}

/// Returns every locale embedded in the application, in deterministic order.
#[must_use]
pub fn supported_locales() -> Vec<String> {
    let mut locales = Locales::iter()
        .filter_map(|path| locale_from_resource_path(path.as_ref()).map(str::to_owned))
        .collect::<Vec<_>>();
    locales.sort_unstable();
    locales.dedup();
    locales
}

/// Resolves a requested BCP-47 tag to an embedded locale.
///
/// Exact matches are preferred, then a same-language translation, followed by
/// the default English translation.
#[must_use]
pub fn resolve_locale(requested_locale: Option<&str>) -> String {
    let locales = supported_locales();
    let requested_locale = requested_locale.map(normalise_locale_tag);

    if let Some(requested_locale) = requested_locale.as_deref() {
        if let Some(locale) =
            locales.iter().find(|locale| locale.eq_ignore_ascii_case(requested_locale))
        {
            return locale.clone();
        }

        let requested_language = requested_locale.split('-').next().unwrap_or_default();
        if let Some(locale) = locales.iter().find(|locale| {
            locale
                .split('-')
                .next()
                .is_some_and(|language| language.eq_ignore_ascii_case(requested_language))
        }) {
            return locale.clone();
        }
    }

    locales
        .into_iter()
        .find(|locale| locale == DEFAULT_LOCALE)
        .unwrap_or_else(|| DEFAULT_LOCALE.to_owned())
}

/// Resolves the process locale from `LC_ALL`, `LANG`, or the English fallback.
#[must_use]
pub fn system_locale() -> String {
    let locale = std::env::var("LC_ALL")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| std::env::var("LANG").ok().filter(|value| !value.is_empty()));
    resolve_locale(locale.as_deref())
}

/// Formats a translated message without interpolation arguments.
///
/// # Errors
///
/// Returns an error if the embedded resource is invalid, the message is absent,
/// or Fluent cannot format the message.
pub fn translate(locale: &str, message_id: &str) -> Result<String, LocalisationError> {
    let locale = resolve_locale(Some(locale));
    let resource_path = format!("{locale}/{RESOURCE_NAME}");
    let Some(resource) = Locales::get(&resource_path) else {
        return Err(LocalisationError::InvalidResource { locale });
    };
    let source = String::from_utf8_lossy(resource.data.as_ref()).into_owned();
    let resource = FluentResource::try_new(source)
        .map_err(|_| LocalisationError::InvalidResource { locale: locale.clone() })?;
    let language: LanguageIdentifier = locale
        .parse()
        .map_err(|_| LocalisationError::InvalidResource { locale: locale.clone() })?;
    let mut bundle = FluentBundle::new(vec![language]);
    bundle
        .add_resource(resource)
        .map_err(|_| LocalisationError::InvalidResource { locale: locale.clone() })?;

    let message =
        bundle.get_message(message_id).ok_or_else(|| LocalisationError::MissingMessage {
            locale: locale.clone(),
            message_id: message_id.to_owned(),
        })?;
    let pattern = message.value().ok_or_else(|| LocalisationError::MissingMessageValue {
        locale: locale.clone(),
        message_id: message_id.to_owned(),
    })?;
    let mut errors = Vec::new();
    let translated = bundle.format_pattern(pattern, None, &mut errors).into_owned();
    if errors.is_empty() {
        Ok(translated)
    } else {
        Err(LocalisationError::Formatting { locale, message_id: message_id.to_owned() })
    }
}

fn locale_from_resource_path(path: &str) -> Option<&str> {
    let (locale, filename) = path.split_once('/')?;
    (filename == RESOURCE_NAME).then_some(locale)
}

fn normalise_locale_tag(tag: &str) -> String {
    tag.split(['.', '@']).next().unwrap_or_default().replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::{LocalisationError, resolve_locale, supported_locales, translate};

    #[test]
    fn embeds_the_default_and_serbian_latin_translations() {
        let locales = supported_locales();

        assert!(locales.contains(&"en-US".to_owned()));
        assert!(locales.contains(&"sr-Latn".to_owned()));
    }

    #[test]
    fn prefers_an_exact_locale_match() {
        assert_eq!(resolve_locale(Some("sr-Latn")), "sr-Latn");
    }

    #[test]
    fn falls_back_to_the_same_language() {
        assert!(resolve_locale(Some("sr-RS.UTF-8")).starts_with("sr-"));
    }

    #[test]
    fn falls_back_to_english_for_an_unsupported_language() {
        assert_eq!(resolve_locale(Some("de-DE")), "en-US");
    }

    #[test]
    fn translates_a_cli_message() {
        assert_eq!(
            translate("sr-Latn", "cli-sor-not-ready").expect("translation should exist"),
            "Podrška za čitanje SOR fajlova još nije dostupna."
        );
    }

    #[test]
    fn every_embedded_catalog_has_each_required_message() {
        for locale in supported_locales() {
            for message_id in ["app-name", "cli-sor-not-ready"] {
                translate(&locale, message_id).unwrap_or_else(|error| {
                    panic!("locale `{locale}` must provide `{message_id}`: {error}")
                });
            }
        }
    }

    #[test]
    fn reports_a_missing_message() {
        assert_eq!(
            translate("en-US", "does-not-exist"),
            Err(LocalisationError::MissingMessage {
                locale: "en-US".to_owned(),
                message_id: "does-not-exist".to_owned(),
            })
        );
    }
}
