# Contributing to pawn-crypto

Discuss bugs and feature requests through the issue templates. Use synthetic
passwords and keys in reproductions; never attach real credentials, password
hashes or an unredacted server log. Report vulnerabilities privately as described
in [SECURITY.md](SECURITY.md).

## Development

Rust is pinned by `rust-toolchain.toml`. Install the 32-bit targets and a Linux
multilib linker when working on Linux:

```sh
rustup show
rustup target add i686-unknown-linux-gnu i686-pc-windows-msvc
cargo fmt --all -- --check
cargo test --locked --target i686-unknown-linux-gnu --lib
cargo clippy --locked --target i686-unknown-linux-gnu --all-targets -- -D warnings
cargo clippy --locked --target i686-unknown-linux-gnu --all-targets --features samp-only -- -D warnings
python3 .github/scripts/check_pawn_encoding.py
```

`./scripts/build-linux.sh` builds Linux and Windows from Linux;
`./scripts/build-windows.sh` does the reverse from Git Bash. See [README.md](README.md)
for prerequisites. `./scripts/licenses.sh` inventories the locked dependencies.
The project's license is still undecided; dependency acceptance in `about.toml`
is separate from licensing this repository.

## Pawn validation

Use the open.mp Pawn compiler with the SA-MP standard includes. The compiler
does not require the open.mp standard library. Compile every example and
fixture, requiring both a successful exit and no warnings:

```sh
python3 scripts/check-pawn.py --compiler /path/to/pawncc --stdlib /path/to/sa-mp/includes
```

For isolated runtime tests, point the same command at an installed Linux open.mp
server directory. It creates disposable server directories under the system's
temporary directory and runs both native and legacy loading modes:

```sh
python3 scripts/check-pawn.py --compiler /path/to/qawno/pawncc --stdlib /path/to/sa-mp/includes --server /path/to/omp-server-directory
```

The runner uses the server's standard components, never its configuration,
plugins, gamemodes or credentials. It binds only to loopback and retains the
test logs for inspection. Windows runtime validation and tests on a SA-MP
server remain separate from this Linux open.mp check.

## Implementation conventions

- Keep Rust code, logs, includes and documentation in en-US.
- Define plugin state and lifecycle in `src/plugin.rs`; group native implementations
  by domain under `src/natives/`. Keep `src/lib.rs` focused on registration.
- Add a native to its Rust module, the registration list, the include template
  and a runnable example. Set its explicit `#[native(name = "...")]` name.
- Edit `include/crypto.inc.in`, then build. Both `.inc` files are generated and
  committed; `crypto_omp.inc` derives its aliases from the base include.
- Pawn files use ASCII, LF and no BOM. Document every native with `@param` and
  `@return`; use `bool:` for boolean results and `sizeof(output)` for buffers.
- Keep `Cargo.toml` as the source of the package version; generated includes
  carry it automatically. Update [CHANGELOG.md](CHANGELOG.md) under Unreleased.
- Use established cryptographic libraries and known-answer tests. Preserve raw
  Pawn bytes and use explicit conversions between numeric types.
- Keep expensive work off the server thread. Use rust-samp's main-thread queue,
  AMX identity and callback helpers; respect callback re-entry and script unload.
- Bound worker concurrency, input sizes and total pending operations. Do not log
  passwords, secret keys or stored password hashes.
- Review direct and transitive dependency licenses with cargo-about after a
  dependency change. The offline report can contain synthesized license texts;
  original attribution notices still require review before distribution.

## Commits and pull requests

Use Conventional Commits, with Portuguese commit messages. Keep a complete,
verified task in one commit; separate steps belong inside a pull request and
should be consolidated before merging when they form one change.

Commits include `Signed-off-by` (`git commit -s`). Maintainer commits are also
GPG-signed (`git commit -S -s`); the DCO sign-off and cryptographic signature are
independent.

Describe the problem, resulting behavior, verification and remaining limitations
in the pull request. Do not report checks that were not run. Tags and releases
require maintainer approval of the version and changelog before publication.

## CI

GitHub Actions checks Linux GNU and Windows MSVC builds, unit tests, Clippy,
formatting, generated includes, Pawn encoding and dependency licenses. A separate
workflow runs RustSec audits on changes and weekly. Dependabot tracks Cargo
(including transitive dependencies) and GitHub Actions.

CodeQL is configured for Rust and Actions. It runs automatically for public
repositories. For a private repository, enable `ENABLE_CODEQL=true` only after
CodeQL entitlement and code scanning are available. Repository access settings,
branch protection and vulnerability-reporting settings are managed separately.
