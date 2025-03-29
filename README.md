<!-- insert
---
title: "Rusqlite Snapshot Testing"
date: 2025-03-29T17:32:05
description: "Snapshot testing tool for rusqlite"
tags:
- Rust
- SQLite
- Library
---
end_insert -->

<!-- remove -->
<div align="center">

# Rusqlite Snapshot Testing
<!-- end_remove -->

<!-- insert
{{< github_badge >}}

{{< rawhtml >}}
<div class="badges">
{{< /rawhtml >}}
end_insert -->

[![docs.rs](https://img.shields.io/docsrs/rusqlite-snapshot-testing)][docs]
[![Crates.io](https://img.shields.io/crates/v/rusqlite-snapshot-testing)][cio]
<!-- insert
{{< rawhtml >}}
end_insert -->
</div>
<!-- insert
{{< /rawhtml >}}
end_insert -->

<!-- rustdoc start -->

Tool to perform snapshot testing on an SQLite database, using [rusqlite][].

The goal is to expose both data and the schema in the snapshots. It is also compatible with [Insta Snapshots][insta]

**This is experimental software, expect breaking changes between 0.x versions, consistent with the semver rules for Rust.**

## Background reading on snapshot testing.

- https://ianthehenry.com/posts/my-kind-of-repl/
- https://tigerbeetle.com/blog/2024-05-14-snapshot-testing-for-the-masses/
- https://blog.janestreet.com/the-joy-of-expect-tests/

[cio]: https://crates.io/crates/rusqlite-snapshot-testing
[docs]: https://docs.rs/rusqlite-snapshot-testing
[insta]: https://insta.rs
[rusqlite]: https://crates.io/crates/rusqlite
