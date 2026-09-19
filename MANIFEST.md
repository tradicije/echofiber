# EchoFiber Manifest

EchoFiber began with a familiar field frustration: a measurement is made with
equipment you know, on fibre you are responsible for, yet the result is locked
inside a file that only a particular vendor's software can properly read.

That frustration is sharper because the Standard OTDR Record (SOR) is the
common interchange format for OTDR measurements. It was created so traces can
move between instruments and tools, rather than remain trapped in a single
vendor's private format. Yet its formal specification, Telcordia SR-4731, is
not openly published: access to the full specification is proprietary. In
practice, that leaves technicians dependent on vendor viewers, paid software,
or independently reconstructed parsers just to inspect a trace, its events,
and its metadata.

The problem is not that every viewer has a price tag. Free viewers and
independent tools exist. The problem is that a standard measurement record
should not make the person holding it dependent on a vendor's closed
application or an undocumented implementation in order to see and understand
their own work. EchoFiber exists to make that dependence smaller.

## Standards and legal context

The format discussed here is the Telcordia document
[SR-4731, *Optical Time Domain Reflectometer (OTDR) Data Format*, Issue 2](https://www.intertekinform.com/en-gb/standards/sr-4731-issue-2-1063022_saig_tel_tel_2472829/).
Its commercial availability is itself part of the problem EchoFiber addresses:
an industry interchange format should be understandable in practice by the
people who create and retain its records.

EchoFiber is an independently written program. Its work is limited to lawful
interoperability, permitted samples, and careful observation of file behaviour;
it does not copy a vendor's code or claim access to the SR-4731 text. In Serbia,
[Article 47 of the Law on Copyright and Related Rights](https://www.paragraf.rs/propisi/zakon_o_autorskom_i_srodnim_pravima.html)
sets conditions under which reproducing or translating a computer program's
code to obtain interoperability information can be permitted. That provision
concerns computer programs, comes with strict conditions and limits, and is not
a blanket permission for every kind of reverse engineering. EchoFiber treats it
as context for its interoperability purpose, not as legal advice or a claim of
legal clearance for any particular input, implementation, or jurisdiction.

An OTDR trace is not merely a proprietary artefact. It is a record of physical
infrastructure, of time spent in the field, and often of a decision that has to
be explained long after the instrument has been packed away. The person who
made that measurement should be able to inspect it, preserve it, and use it
without being forced into one vendor's application, operating system, account,
or upgrade cycle.

I work with OTDRs in the field. EchoFiber is my attempt to make one small,
practical part of that work more understandable and more durable. Its purpose
is simple: read SOR measurements faithfully and export their useful contents
to ordinary, documented formats that people can keep using.

## The measurement belongs to the people who made it

Fibre measurements can contain sensitive names, locations, cable labels, and
customer information. They must be handled carefully. But careful handling is
not the same as surrendering control. EchoFiber does not modify source files,
does not send them over the network by default, and aims to produce local,
open exports with explicit units.

The original SOR file remains the source record. Exports make it easier to
inspect and exchange what is known; they do not erase the original, nor do they
pretend that an uncertain interpretation is a fact. Unknown and vendor-specific
blocks are worth preserving because future understanding should not require
recovering data that a tool casually discarded.

## Interoperability is a practical freedom

This project is not built to declare every SOR file compatible on faith.
Compatibility is earned through permitted, anonymised samples and comparison
with the instruments that created them. We document what has been verified,
state what remains unknown, and improve the parser through evidence.

That discipline is not a limitation of the project; it is respect for field
work. A plausible but wrong distance, loss, or event table is worse than an
honest diagnostic. Software used around physical infrastructure must make its
limits visible.

Open JSON and CSV exports are part of this promise. They make data available to
scripts, reports, archival systems, and future tools without requiring anyone
to reverse-engineer a private database or keep obsolete software alive just to
open their own measurement.

## Why the AGPL-3.0-or-later licence

EchoFiber is free software because its users should be free to study it, run
it, change it, and share it. “Free” here means freedom, not necessarily zero
cost. People who rely on a tool should be able to understand the code that
interprets their measurements and should be able to adapt it when their work
demands it.

The GNU Affero General Public License, version 3 or later, protects those
freedoms when software is distributed and when it is offered to users over a
network. If someone improves EchoFiber and provides that improved version as a
service, the people using the service deserve access to the corresponding
source code. The same freedom that made the project possible should remain
available to the community that depends on it.

This is also a refusal of vendor lock-in. No company should be able to take a
community tool, add a closed layer around it, and turn the community's own work
back into a dependency. Businesses may use, support, host, and improve
EchoFiber. The condition is straightforward: the freedom received with the
software must remain available to its users.

## A small tool for a real corner of the world

EchoFiber does not claim to solve the whole OTDR ecosystem. It begins with a
specific problem in a specific trade, where open tools are often missing and
where proprietary formats outlive the software sold to open them.

Small tools matter when they give people control over a part of their working
lives. This project will be useful if it helps a technician, engineer,
contractor, researcher, or archive understand and retain a measurement they
already have.

You do not need to be a Rust developer to help. You can contribute a permitted
and anonymised sample, document instrument behaviour, test an export, report a
problem, translate messages, improve documentation, or share knowledge from
the field. EchoFiber should grow from the experience of the people who use
OTDRs, not from assumptions made at a distance.

The goal is modest and demanding: a trustworthy local tool, open records, and
the freedom to understand the measurements on which real work depends.

---

The wider philosophy behind this project is described at
[Dimitrium: Digital Philosophy](https://dimitrium.org/en/philosophy/).
