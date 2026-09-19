# Contributing

Thank you for contributing. The project is at an early stage, so small,
verifiable steps are more valuable than large rewrites.

## Before submitting a change

- Open an issue or discussion for a substantial change to the format, parser behaviour, or API.
- Do not modify original SOR inputs; the parser must be read-only.
- Add a test demonstrating every fixed problem.
- Keep units explicit (`m`, `km`, `dB`, `ns`) and never assume them.
- Do not copy code or samples without permission and a clear licence/provenance.

## SOR samples

Do not submit files that disclose locations, customer identities, fibre labels,
or other confidential data. For every permitted sample, provide:

1. the instrument manufacturer and model, if permitted;
2. the firmware version, if known;
3. confirmation that you may share it under the repository terms;
4. the expected behaviour or observed issue.

Small, anonymised fixtures are preferred. Large or restrictively shareable
samples will not be added to the public repository.

## Localisation

Translations are welcome. Add a Fluent catalog at
`locales/<BCP-47-language-tag>/messages.ftl`, copy every message identifier
from the English reference catalog, and translate values only. Do not include
OTDR measurements, customer information, file paths, or other input data in a
translation.

Run the standard checks before submitting a translation. They verify that every
catalog provides every required message. See
[docs/localization.md](docs/localization.md) for the full format and locale
selection rules.

## Contribution licence

By submitting a contribution, you confirm that you have the right to submit it
and license it under this project's `AGPL-3.0-or-later` terms.

## Style and quality

- Parsing must check boundaries before every binary read.
- Unknown blocks are data to preserve, not grounds for silently discarding a file.
- Errors must explain what is wrong and, where possible, where it was found.
- Document assumptions, especially where instrument behaviour differs.
