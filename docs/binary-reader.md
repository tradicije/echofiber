# Binary reader

`echofiber_core::binary::BinaryReader` is the only intended starting point for
reading untrusted SOR bytes. It borrows an immutable byte slice, performs no
allocation, and never mutates the source input.

Every read and skip operation checks its range before indexing. Failed reads do
not advance the cursor. `BinaryReadError` includes the absolute byte offset,
the requested length, and the bytes remaining, so callers can turn parsing
failures into useful diagnostics.

`read_sub_reader(length)` is used for bounded binary regions such as future SOR
blocks. The parent cursor advances only after the region is validated, while
the returned reader remains unable to read outside that region. Its errors keep
the original file's absolute offsets.

Endianness is explicit in every multi-byte method. A parser must choose the
method specified by the relevant SOR field; it must not infer endianness from
the machine running `EchoFiber`.
