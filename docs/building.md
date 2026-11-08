# Build and validation

## Toolchain and prerequisites

Use the Rust version pinned in
[rust-toolchain.toml](https://github.com/NullSablex/pawn-crypto/blob/master/rust-toolchain.toml).
`Cargo.toml` declares the same minimum. Both supported server targets are 32-bit:
`i686-unknown-linux-gnu` and `i686-pc-windows-msvc`.

Linux builds need a 32-bit GNU linker and development libraries. The helper
scripts also use `jq` and `rg`. For Debian/Ubuntu:

```sh
sudo apt install gcc-multilib jq ripgrep clang lld
rustup show
```

MSVC builds need a Windows SDK and MSVC tools, or `cargo-xwin` for cross-compiling
from Linux. The Linux script installs missing `cargo-xwin` and Rust targets.
It can download the Microsoft toolchain components on first use.

## Helper scripts

### From Linux

```sh
./scripts/build-linux.sh
PROFILE=dev ./scripts/build-linux.sh
```

### From Windows / Git Bash

Run with the MSVC toolchain available, for example from a developer shell.
Linux output additionally needs Rust in WSL, or Docker Desktop and `cross`.

```sh
./scripts/build-windows.sh
./scripts/build-windows.sh --wsl
./scripts/build-windows.sh --docker
```

The default profile is `release`; `PROFILE=dev` uses Cargo's development profile.
The scripts collect `dist/pawn_crypto.so` and `dist/pawn_crypto.dll`.
Cargo's lockfile is required and respected. Generate
[license reports](licenses.md) separately with `cargo-about`.

## Direct Cargo builds

```sh
rustup target add i686-unknown-linux-gnu i686-pc-windows-msvc
cargo build --locked --release --target i686-unknown-linux-gnu
```

The native Windows equivalent, with MSVC available, is:

```sh
cargo build --locked --release --target i686-pc-windows-msvc
```

Direct outputs are `target/i686-unknown-linux-gnu/release/libpawn_crypto.so` and
`target/i686-pc-windows-msvc/release/pawn_crypto.dll`. Rename the Linux artifact
to `pawn_crypto.so` for the documented installation commands.

Default builds expose both server interfaces. Add `--features samp-only` for
legacy exports only. Windows GNU builds require `samp-only`; they do not provide
the native open.mp MSVC ABI. Prefer the documented MSVC target for Windows.

## Validate Rust and Pawn

```sh
cargo fmt --all -- --check
cargo test --locked --target i686-unknown-linux-gnu --lib
cargo clippy --locked --target i686-unknown-linux-gnu --all-targets -- -D warnings
cargo clippy --locked --target i686-unknown-linux-gnu --all-targets --features samp-only -- -D warnings
cargo xtask check-pawn
```

Compile all Pawn examples and fixtures using the open.mp compiler with SA-MP's
standard includes:

```sh
cargo xtask pawn --compiler /path/to/qawno/pawncc --stdlib /path/to/sa-mp/includes
```

Add `--server /path/to/omp-server-directory` to run isolated Linux open.mp tests
in native and legacy modes. The runner uses only standard server components,
creates its own configuration, binds to loopback and retains logs under its
temporary workspace. It does not use your server's configuration or credentials.

## Documentation site

Documentation uses Zensical, pinned in `requirements-docs.txt`. Install `uv`,
then build and validate the documentation from the repository root:

```sh
cargo xtask docs
uv tool run --python 3.13 --from "$(cat requirements-docs.txt)" zensical serve
```

`cargo xtask docs` checks API freshness, runs Zensical in strict mode, and
validates local Markdown targets and rendered anchors. The output is
`dist/docs/`; the preview server uses the same configuration. `uv` runs the
pinned generator in an isolated tool environment with a Python 3.13 runtime.
The documentation workflow uploads the generated files as an artifact. After
successful builds on `master`, it publishes them to
[GitHub Pages](https://nullsablex.github.io/pawn-crypto/). Pull requests only
build and validate the documentation.

The API page is generated from `include/crypto.inc` and `crypto_omp.inc`. After
editing a native's include documentation and building the plugin, regenerate it:

```sh
cargo xtask gen-api
cargo xtask gen-api --check
```

The generator is part of the standalone Rust tooling crate under `xtask/`.
Its dependencies are locked in `xtask/Cargo.lock`, independently of the plugin.
CI checks the committed reference, which also renders directly on GitHub.
Navigation, repository links, colors and light/dark themes are configured in
`zensical.toml`. Documentation pages remain Markdown files under `docs/`.

The Pawn runner also accepts `--docs` to compile complete Pawn examples in the
README and guides. Validation, reference generation and runtime tests use Rust
through `cargo xtask`.

## Contributing

See [CONTRIBUTING.md](https://github.com/NullSablex/pawn-crypto/blob/master/CONTRIBUTING.md)
for module layout, native registration, commit conventions and pull request
requirements. Package version changes start in `Cargo.toml`; do not duplicate
the current version in documentation.
