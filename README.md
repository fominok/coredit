# Coredit
[![Latest Release][crates-io-badge]][crates-io-url]
[![Documentation][docs-rs-badge]][docs-rs-url]

🖋 A text editor buffer component based on [Ropey][ropey-url]
with multiselection heavy lifting.

## What Coredit is
While there are crates providing data structures to keep text
maintaining performance on random access the other common part
is how a cursor should act. Inspired by [Kakoune][kakoune-url],
Coredit attempts to simplify creation of a custom text editor
with sensible defaults, including multiple selections. (Those
who don't like multiple selections can use only one!)

## What Coredit is not
It's not an editor: at least you need to attach a keyboard
reader and to draw current state of the buffer. However, there
is a minimal editor in `examples` directory
(`cargo run --example edi`).

Coredit doesn't try to be full-featured: it is busy storing
text, doing manual changes and controlling selections. LSP
integrations, Lua scripting, highlighting and `M-x doctor`
could be main features of the text editor you made, but they
should not belong to the buffer core.

## State of the crate
By tradition, let's call this a _very WIP_. Seriously,
there is a known room for improvement by means of performance,
also API may become heavier if the idea of "minimal core" is
not that good.

[crates-io-badge]: https://img.shields.io/crates/v/coredit.svg
[crates-io-url]: https://crates.io/crates/coredit
[docs-rs-badge]: https://docs.rs/coredit/badge.svg
[docs-rs-url]: https://docs.rs/coredit
[ropey-url]: https://github.com/cessen/ropey
[kakoune-url]: https://kakoune.org
