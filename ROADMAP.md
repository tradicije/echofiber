# Roadmap

This roadmap describes the order of work, not promised delivery dates.

## Phase 0 — Foundations

- [x] AGPL-3.0-or-later licence and basic project policies.
- [x] Choose the CLI package name and Rust workspace structure (`echofiber`,
  `echofiber-core`, and `echofiber-cli`).
- [x] Provide contributor-extensible localisation for user-facing messages.
- [ ] Define the sample, anonymisation, and data-provenance policy.
- [ ] Collect publicly redistributable SOR fixtures and record their licences.

## Phase 1 — Reliable binary foundation

- [x] Checked, read-only binary reader with bounded nested regions and
  absolute-offset errors.
- [ ] Read-only parser for the block map and strict file-boundary validation.
- [ ] Diagnostics for corrupted, truncated, and unknown blocks.
- [ ] Structures for preserving unknown blocks unchanged.
- [ ] Unit and fuzz tests using synthetic malformed inputs.

## Phase 2 — First export

- [x] Define preliminary, versioned schemas for `metadata.json`, `trace.csv`,
  and `events.json`; implementation against actual SOR input remains pending.
- [ ] `inspect`, `convert`, and `validate` CLI commands.

## Phase 3 — Field validation

- [ ] Golden tests for anonymised EXFO captures.
- [ ] Golden tests for anonymised Orientek captures.
- [ ] Compare outputs with the instruments' displays and reports.
- [ ] Document confirmed vendor differences and support boundaries.

## Phase 4 — Broader ecosystem

- [ ] Instructions for submitting samples from other instruments.
- [ ] A stable library and distribution package.
- [ ] Optional local viewer.
- [ ] Assess whether a dedicated container format is necessary; JSON/CSV remain the primary export.
