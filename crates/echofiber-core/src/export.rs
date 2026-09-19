//! Versioned, open export models for `EchoFiber`.
//!
//! These types describe exported data only. They do not imply that every field
//! is available in every SOR file or supported instrument.

use std::io::Write;

use serde::Serialize;

/// Current schema version for JSON metadata exports.
pub const METADATA_SCHEMA_VERSION: &str = "0.1.0";

/// Current schema version for JSON event exports.
pub const EVENTS_SCHEMA_VERSION: &str = "0.1.0";

/// Current column schema version for CSV trace exports.
pub const TRACE_CSV_SCHEMA_VERSION: &str = "0.1.0";

/// The contents of a `metadata.json` export.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetadataDocument {
    /// Version of this document's schema.
    pub schema_version: &'static str,
    /// Information about the parsed input, excluding its contents.
    pub source: SourceMetadata,
    /// Fibre identification reported by the instrument, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fibre: Option<FibreMetadata>,
    /// Acquisition parameters reported by the instrument, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition: Option<AcquisitionMetadata>,
    /// Instrument identity reported by the instrument, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentMetadata>,
}

impl MetadataDocument {
    /// Creates a metadata document using the current schema version.
    #[must_use]
    pub const fn new(source: SourceMetadata) -> Self {
        Self {
            schema_version: METADATA_SCHEMA_VERSION,
            source,
            fibre: None,
            acquisition: None,
            instrument: None,
        }
    }
}

/// Non-sensitive technical information about an input file.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SourceMetadata {
    /// Format of the input file. Current exports use `"sor"`.
    pub format: &'static str,
    /// Input size in bytes.
    pub byte_length: u64,
}

/// Fibre identification supplied by an instrument.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct FibreMetadata {
    /// Instrument-reported fibre identifier; may contain sensitive data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Instrument-reported cable identifier; may contain sensitive data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_id: Option<String>,
}

/// Parameters describing an OTDR acquisition.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct AcquisitionMetadata {
    /// Nominal wavelength in nanometres.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wavelength_nm: Option<u32>,
    /// Pulse width in nanoseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pulse_width_ns: Option<u64>,
    /// Acquisition duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_s: Option<f64>,
}

/// Identity information reported by an OTDR instrument.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct InstrumentMetadata {
    /// Instrument manufacturer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Instrument model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Instrument firmware version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
}

/// One row in a `trace.csv` export.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TraceSample {
    /// Distance from the trace origin, in metres.
    pub distance_m: f64,
    /// Recorded trace signal level, in decibels.
    pub signal_db: f64,
}

/// The contents of an `events.json` export.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventsDocument {
    /// Version of this document's schema.
    pub schema_version: &'static str,
    /// Events in their instrument-reported order.
    pub events: Vec<Event>,
}

impl EventsDocument {
    /// Creates an event document using the current schema version.
    #[must_use]
    pub fn new(events: Vec<Event>) -> Self {
        Self { schema_version: EVENTS_SCHEMA_VERSION, events }
    }
}

/// A single event reported by an OTDR instrument.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Event {
    /// One-based position in the event table.
    pub index: u32,
    /// Distance from the trace origin, in metres.
    pub distance_m: f64,
    /// Event classification as reported or normalised by the parser.
    pub event_type: String,
    /// Insertion loss, in decibels, when reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_db: Option<f64>,
    /// Reflectance, in decibels, when reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflectance_db: Option<f64>,
    /// Fibre attenuation after the event, in decibels per kilometre, when reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation_db_per_km: Option<f64>,
}

/// Writes a `metadata.json` document as pretty-printed JSON.
///
/// # Errors
///
/// Returns an error if JSON serialization or writing to the destination fails.
pub fn write_metadata_json<W: Write>(
    writer: W,
    document: &MetadataDocument,
) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, document)
}

/// Writes an `events.json` document as pretty-printed JSON.
///
/// # Errors
///
/// Returns an error if JSON serialization or writing to the destination fails.
pub fn write_events_json<W: Write>(writer: W, document: &EventsDocument) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, document)
}

/// Writes trace samples to `trace.csv` with stable, unit-bearing headers.
///
/// # Errors
///
/// Returns an error if CSV serialization or writing to the destination fails.
pub fn write_trace_csv<W: Write>(writer: W, samples: &[TraceSample]) -> csv::Result<()> {
    let mut csv_writer = csv::Writer::from_writer(writer);
    for sample in samples {
        csv_writer.serialize(sample)?;
    }
    csv_writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        Event, EventsDocument, MetadataDocument, SourceMetadata, TraceSample, write_events_json,
        write_metadata_json, write_trace_csv,
    };

    #[test]
    fn writes_minimal_metadata_without_sensitive_fields() {
        let document = MetadataDocument::new(SourceMetadata { format: "sor", byte_length: 42 });
        let mut output = Vec::new();

        write_metadata_json(&mut output, &document).expect("metadata serialization should succeed");

        assert_eq!(
            String::from_utf8(output).expect("JSON must be UTF-8"),
            "{\n  \"schema_version\": \"0.1.0\",\n  \"source\": {\n    \"format\": \"sor\",\n    \"byte_length\": 42\n  }\n}"
        );
    }

    #[test]
    fn writes_trace_with_documented_headers() {
        let samples = [TraceSample { distance_m: 12.5, signal_db: -3.25 }];
        let mut output = Vec::new();

        write_trace_csv(&mut output, &samples).expect("trace serialization should succeed");

        assert_eq!(
            String::from_utf8(output).expect("CSV must be UTF-8"),
            "distance_m,signal_db\n12.5,-3.25\n"
        );
    }

    #[test]
    fn writes_events_in_instrument_order() {
        let document = EventsDocument::new(vec![Event {
            index: 1,
            distance_m: 100.0,
            event_type: "splice".to_owned(),
            loss_db: Some(0.12),
            reflectance_db: None,
            attenuation_db_per_km: None,
        }]);
        let mut output = Vec::new();

        write_events_json(&mut output, &document).expect("event serialization should succeed");

        let output = String::from_utf8(output).expect("JSON must be UTF-8");
        assert!(output.contains("\"events\": ["));
        assert!(output.contains("\"event_type\": \"splice\""));
        assert!(!output.contains("reflectance_db"));
    }
}
