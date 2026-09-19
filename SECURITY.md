# Security Policy

## Supported versions

Until the first stable release, security fixes are applied to the main development
branch.

## Reporting a vulnerability

Do not open a public issue for a vulnerability that could lead to code execution,
denial of service, data disclosure, or file corruption. Instead, submit a private
report to the repository owner through the security contact listed on the hosting
platform.

Include a minimal reproducer, tool version, operating system, and impact
description. Do not send confidential measurements; provide an anonymised or
synthetic example whenever possible.

## Security model

An SOR file is untrusted binary input. The parser must:

- check offsets, lengths, and overflows before allocating or reading;
- limit memory and time consumption;
- treat textual fields as untrusted content;
- never execute contents from an SOR file;
- not send inputs, metadata, or telemetry across the network by default.

## Disclosure

Confirmed vulnerabilities will receive a fix, regression test, and public notice
after a safe version is available.
