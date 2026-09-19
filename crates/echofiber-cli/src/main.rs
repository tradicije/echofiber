//! Command-line interface for `EchoFiber`.

fn main() {
    let locale = echofiber_core::i18n::system_locale();
    let application_name = echofiber_core::i18n::translate(&locale, "app-name")
        .unwrap_or_else(|_| "EchoFiber".to_owned());
    let unavailable_message = echofiber_core::i18n::translate(&locale, "cli-sor-not-ready")
        .unwrap_or_else(|_| "SOR reading support is not available yet.".to_owned());

    println!("{application_name} {}", echofiber_core::version());
    println!("{unavailable_message}");
}
