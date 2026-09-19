# Localisation

`EchoFiber` uses [Project Fluent](https://projectfluent.org/) message files.
Every translation is embedded in the released executable; the program makes no
network request to load a language.

## Adding a translation

1. Create `locales/<BCP-47-tag>/messages.ftl`, for example
   `locales/de-DE/messages.ftl`.
2. Copy every message identifier from `locales/en-US/messages.ftl`.
3. Translate values only. Keep the identifier before `=` unchanged.
4. Run `cargo test --workspace` and submit the translation with its language
   tag and reviewer information.

The build discovers locale directories automatically. No Rust registry or
source-code modification is needed for a new `messages.ftl` file.

## Locale selection

The CLI uses `LC_ALL`, then `LANG`. It accepts common system forms such as
`sr_RS.UTF-8`, normalises them to BCP-47 style, and chooses in this order:

1. exact embedded locale;
2. another embedded locale with the same language;
3. `en-US`.

English (`en-US`) is the required reference catalog. A message must remain
clear without relying on the surrounding terminal output. Do not put OTDR
measurements, customer details, file paths, or other input data into a
translation catalog.
