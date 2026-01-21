## 0.26.0

Released 2026-01-21

### Added

- `ORM` layer which abstracts `SeaQuery` (https://github.com/SeaQL/sea-query) and provides
  a database agnostic mechanism to build SQL queries ergonomically while maintaining type
  safety and avoiding database specific dependencies in WASM guest components

### Changed

- Fixed outgoing Http
- Updated shared workflow location
- Modified `default` in-memory implementation in the `wasi-sql` crate to preserve database
  connection for the duration of the host lifetime
- Modified `sql` example to use the `ORM` constructs

---

Release notes for previous releases can be found on the respective release
branches of the repository.

<!-- ARCHIVE_START -->

- [0.25.x](https://github.com/augentic/qwasr/blob/release-0.25.0/RELEASES.md)
- [0.23.x](https://github.com/augentic/qwasr/blob/release-0.23.0/RELEASES.md)
- [0.22.x](https://github.com/augentic/qwasr/blob/release-0.22.0/RELEASES.md)
- [0.21.x](https://github.com/augentic/qwasr/blob/release-0.21.0/RELEASES.md)
- [0.20.x](https://github.com/augentic/qwasr/blob/release-0.20.0/RELEASES.md)
- [0.20.x](https://github.com/augentic/qwasr/blob/release-0.20.0/RELEASES.md)
- [0.19.x](https://github.com/credibil/wrt/blob/release-0.19.0/RELEASES.md)
- [0.18.x](https://github.com/credibil/wrt/blob/release-0.18.0/RELEASES.md)
- [0.17.x](https://github.com/credibil/wrt/blob/release-0.17.0/RELEASES.md)
- [0.16.x](https://github.com/credibil/wrt/blob/release-0.16.0/RELEASES.md)
- [0.15.x](https://github.com/credibil/wrt/blob/release-0.15.0/RELEASES.md)
