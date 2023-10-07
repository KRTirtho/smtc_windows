# Change Log

All notable changes to this project will be documented in this file.
See [Conventional Commits](https://conventionalcommits.org) for commit guidelines.

## 2023-10-07

### Changes

---

Packages with breaking changes:

 - There are no breaking changes in this release.

Packages with other changes:

 - [`smtc_windows` - `v0.1.1`](#smtc_windows---v011)

---

#### `smtc_windows` - `v0.1.1`

 - **REFACTOR**: use internal wrapper to allow using for both binding and native Rust.
 - **FIX**: upgrade flutter_rust_bridge.
 - **FIX**: update config not updated in initialization causing smtc to not show up.
 - **FEAT**: store smtc enable/disable state.
 - **FEAT**: enable/disable smtc support, clear metadata support and all optional music properties.
 - **FEAT**: hide internal bridge api and add exception for usage on non-windows environment.
 - **FEAT**: added conventional struct (incomplete).
 - **FEAT**(events): button pressed, shuffle and repeat mode events.

