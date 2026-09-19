# EchoFiber

Free, interoperable tooling for reading and exporting OTDR `*.sor` measurements.

The project first converts Standard OTDR Record (SOR) files into open,
everyday-use outputs:

- `metadata.json` for acquisition, fibre identification, and device metadata;
- `trace.csv` for trace samples and their distances;
- `events.json` for the event table recorded by the instrument;
- a report of unrecognised blocks, without discarding their data.

## Status

The project is in its foundation stage. It has a Rust workspace with a core
library, a CLI shell, a checked binary reader, and preliminary versioned export
models; SOR parsing has not yet been implemented. The first supported sources
will be SOR files from EXFO and Orientek OTDR instruments.

The current development schemas for `metadata.json`, `trace.csv`, and
`events.json` are documented in [docs/export-formats.md](docs/export-formats.md).
The binary safety boundary is documented in
[docs/binary-reader.md](docs/binary-reader.md).

## Development

EchoFiber requires Rust 1.85 or later. The standard local checks are:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The `echofiber` command currently confirms the installed application version;
file-reading commands will be added after the binary format foundation is in
place.

## Localisation

The CLI supports embedded Fluent translations, beginning with English (`en-US`)
and Serbian Latin (`sr-Latn`). New translations do not require changing Rust
code: add a BCP-47 locale directory and a `messages.ftl` catalog. See
[docs/localization.md](docs/localization.md) for the contribution format and
locale fallback rules.

## AI-assisted development

AI tools may assist with implementation, documentation, and routine review.
They do not replace technical ownership: the project's architecture, format
decisions, test strategy, and acceptance criteria are defined and reviewed by
people.

OTDR measurements, sample selection, anonymisation, provenance, and validation
against instrument displays and reports are human-performed responsibilities.
No compatibility claim is accepted solely from AI-generated output or an
unverified interpretation of a measurement.

## Goals

- Reliable, read-only reading of SOR files.
- Open JSON and CSV exports with explicit units.
- Preservation of unknown and vendor-specific blocks for forensics and future support.
- Reproducible tests using real, permitted, anonymised samples.
- A local CLI as the first interface; a web interface only after the core stabilises.

## Principles

- The original SOR file is never modified.
- We do not claim compatibility with an instrument that has not been tested.
- Every interpretation of data must be traceable to its input block.
- Measurements may contain sensitive site, fibre, and customer names; samples are not published without permission and anonymisation.

## Licence

Code and documentation are licensed under the GNU Affero General Public License,
version 3 or later (`AGPL-3.0-or-later`). The full text is in [LICENSE](LICENSE).

## Contributing and security

See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md),
[ROADMAP.md](ROADMAP.md), and [CHANGELOG.md](CHANGELOG.md).
