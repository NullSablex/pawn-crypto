# Dependency licenses

The project, including its Rust tooling, original Pawn includes, examples and
documentation, is licensed under
[Apache-2.0](https://github.com/NullSablex/pawn-crypto/blob/master/LICENSE).
Separately identified third-party material retains its own notices and terms.
The dependency report inventories those dependencies; it does not replace the
project license or their attribution requirements.

Contributors retain ownership of their work and grant rights under Apache-2.0.
Each commit requires a
[DCO 1.1](https://github.com/NullSablex/pawn-crypto/blob/master/DCO) sign-off,
which also records acceptance of the
[CLA](https://github.com/NullSablex/pawn-crypto/blob/master/CLA.md) under the
contribution policy. No separate acceptance comment is required.
The [contribution guide](https://github.com/NullSablex/pawn-crypto/blob/master/CONTRIBUTING.md#contribution-rights)
describes acceptance records, employer authorization and pre-merge review.

## Generate the inventory

```sh
cargo fetch --locked
mkdir -p dist
cargo about generate --offline --locked --fail --format json --output-file dist/licenses.json
cargo about generate --offline --locked --fail about.hbs --output-file dist/THIRD-PARTY-LICENSES.html
```

Install `cargo-about` first if needed with `cargo install cargo-about --locked`.
The commands above produce these reports; each release attaches the HTML one:

| Output | Contents |
| --- | --- |
| `dist/licenses.json` | Machine-readable inventory, including dependency versions and license selections. |
| `dist/THIRD-PARTY-LICENSES.html` | Readable inventory with license texts and their dependent crates. |

The scan uses the lockfile and both supported targets, Linux GNU x86 and Windows
MSVC x86. Build, development and transitive dependencies are included. The
unpublished root package is excluded to keep this a third-party report;
its Apache-2.0 license is declared separately in `Cargo.toml`.

## Policy and interpretation

`about.toml` lists accepted dependency licenses and chooses among alternatives,
with MIT preferred when a dependency offers it. Mandatory additional notices
remain part of the inventory. The current selected families are MIT,
BSD-2-Clause and Unicode-3.0; regenerate after any dependency change rather than
assuming this list remains current.

An unaccepted or unresolved license fails the report command. A verified
clarification preserves the original BSD notices in `retour`'s concatenated
license file.

The offline scan uses packaged local sources. Some packages omit a license file
or have text the tool cannot identify, so standard license texts may be
synthesized. This inventory supports review; original copyright notices still
need checking before using it as redistribution attribution material.
