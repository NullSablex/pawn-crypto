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

Contributor tooling (documentation site, API reference generation, release
preparation) is described in
[CONTRIBUTING.md](https://github.com/NullSablex/pawn-crypto/blob/master/CONTRIBUTING.md).
