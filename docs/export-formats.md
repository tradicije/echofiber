# Export formats

The first public export schemas are versioned independently of the program.
Their current development version is `0.1.0`. A value missing from an input is
omitted from JSON rather than represented by a guessed value.

## `metadata.json`

The document contains `schema_version` and a `source` object. `source.format`
is `"sor"`; `source.byte_length` is measured in bytes. Optional groups are
`fibre`, `acquisition`, and `instrument`.

Units in acquisition metadata are embedded in field names:

- `wavelength_nm`: nanometres
- `pulse_width_ns`: nanoseconds
- `duration_s`: seconds

Fibre identifiers and cable IDs originate from the instrument and can be
sensitive. They are not added unless present in the source file.

## `trace.csv`

The column schema is `0.1.0`. It always has a header row and these columns, in
this order:

| Column | Unit | Meaning |
| --- | --- | --- |
| `distance_m` | m | Distance from the trace origin |
| `signal_db` | dB | Recorded trace signal level |

## `events.json`

The document contains `schema_version` and an `events` array in the order
reported by the instrument. Each event has `index`, `distance_m`, and
`event_type`. When available it can also include `loss_db`, `reflectance_db`,
and `attenuation_db_per_km`.

Event names remain strings in this initial schema so vendor-specific labels can
be preserved until real samples establish a safe normalisation vocabulary.
