# EchoFiber

Free, interoperable tooling for reading and exporting OTDR `*.sor` measurements.

The project first converts Standard OTDR Record (SOR) files into open,
everyday-use outputs:

- `metadata.json` for acquisition, fibre identification, and device metadata;
- `trace.csv` for trace samples and their distances;
- `events.json` for the event table recorded by the instrument;
- a report of unrecognised blocks, without discarding their data.

## Status

The project is in its founding stage. The reference implementation has not yet
been written. The first supported sources will be SOR files from EXFO and
Orientek OTDR instruments.

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
