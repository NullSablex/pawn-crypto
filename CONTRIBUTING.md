# Contributing to pawn-crypto

Discuss bugs and feature requests through the issue templates. Use synthetic
passwords and keys in reproductions; never attach real credentials, password
hashes or an unredacted server log. Report vulnerabilities privately as described
in [SECURITY.md](SECURITY.md).

AI assistance is permitted under [AI-POLICY.md](AI-POLICY.md). Contributors
remain responsible for understanding, validating and having the rights to
submit their work.

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
cargo xtask check-pawn
```

`./scripts/build-linux.sh` builds Linux and Windows from Linux;
`./scripts/build-windows.sh` does the reverse from Git Bash. See [the build guide](docs/building.md)
for prerequisites. See [dependency licenses](docs/licenses.md) for direct
`cargo-about` inventory commands.
The project is licensed under [Apache-2.0](LICENSE); dependency acceptance in
`about.toml` is separate from licensing this repository.

## Contribution rights

Read [DCO 1.1](DCO) and the [Contributor License Agreement](CLA.md) before
contributing. You retain copyright in your work; you grant the project and its
recipients the rights described in those documents.

- Sign off every commit with `git commit -s`, using your own name and email.
  Under this contribution policy, your sign-off certifies DCO 1.1 and accepts
  the CLA for that contribution; it is distinct from a GPG signature.
  Co-authors must also supply their own sign-offs. Never add another person's
  certification without their authorization.
- Identify all contributors, third-party material and its licenses. Obtain
  authorization from an employer or other rights holder when necessary.
- Read the CLA before signing off. No separate acceptance comment, form or
  maintainer countersignature is required. Acceptance is recorded in the commit.
- Before merging, review provenance, license compatibility and any required
  rights-holder authorization. CI checks author and co-author sign-off trailers;
  a passing check does not independently establish those legal facts.

Keep changes to the CLA and DCO subject to maintainer review. Do not merge human
contributions before these checks are complete. Automated dependency PRs still
need provenance/license review and valid commit sign-offs; a bot cannot grant
rights on behalf of upstream authors. Repository branch protection must require
`DCO and CLA` check and a maintainer review to enforce these gates on GitHub.

The checker recognizes Dependabot's exact author identity with its standard
`Signed-off-by: dependabot[bot] <support@github.com>` trailer. This is an email
alias allowance, not a sign-off exemption; human co-authors still need their own
sign-offs, and other bot identities are not exempted.

Configure protection for `master` in the repository settings, using the exact
job names declared in the workflows as required checks. Code scanning and code
quality rules require the corresponding GitHub features. In a private repository,
enable CodeQL only when available and set `ENABLE_CODEQL=true` so its required
checks run.

To check a proposed commit range locally:

```sh
cargo xtask check-dco --base BASE_SHA --head HEAD_SHA
```

## Pawn validation

Use the open.mp Pawn compiler with the SA-MP standard includes. The compiler
does not require the open.mp standard library. Compile every example and
fixture, requiring both a successful exit and no warnings:

```sh
cargo xtask pawn --compiler /path/to/pawncc --stdlib /path/to/sa-mp/includes
```

For isolated runtime tests, point the same command at an installed Linux open.mp
server directory. It creates disposable server directories under the system's
temporary directory and runs both native and legacy loading modes:

```sh
cargo xtask pawn --compiler /path/to/qawno/pawncc --stdlib /path/to/sa-mp/includes --server /path/to/omp-server-directory
```

The runner uses the server's standard components, never its configuration,
plugins, gamemodes or credentials. It binds only to loopback and retains the
test logs for inspection. Windows runtime validation and tests on a SA-MP
server remain separate from this Linux open.mp check.

## Implementation conventions

- Keep Rust code, logs, includes and documentation in en-US.
- Use Rust (`cargo xtask`) for validation, generation and test tooling. Do not
  add Python scripts. Zensical's runtime is used only to build documentation;
  its package version is pinned in `requirements-docs.txt`.
- Define plugin state and lifecycle in `src/plugin.rs`; group native implementations
  by domain under `src/natives/`. Keep `src/lib.rs` focused on registration.
- Add a native to its Rust module, the registration list, the include template
  and a runnable example. Set its explicit `#[native(name = "...")]` name.
- Edit `include/crypto.inc.in`, then build. Both `.inc` files are generated and
  committed; `crypto_omp.inc` derives its aliases from the base include.
- Pawn files use ASCII, LF and no BOM. Document every native with `@param` and
  `@return`; use `bool:` for boolean results and `sizeof(output)` for buffers.
- Keep `Cargo.toml` as the source of the package version; generated includes
  carry it automatically. Update the pending pre-release entry in
  [CHANGELOG.md](CHANGELOG.md) while the initial release is being prepared.
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
See [the release guide](docs/releases.md) for tag validation, release channels,
packages and the Release Drafter workflow.

## CI

GitHub Actions checks Linux GNU and Windows MSVC builds, unit tests, Clippy,
formatting, generated includes, Pawn encoding and dependency licenses. Pull
requests also check DCO sign-offs for commit authors and co-authors. A separate
workflow runs RustSec audits on changes and weekly. The docs workflow checks
the generated API reference and builds Zensical in strict mode with link
validation. Dependabot tracks Cargo dependencies in the plugin and xtask,
GitHub Actions and the pinned Zensical package.

Documentation source lives in `docs/`; `zensical.toml` defines navigation and
the theme. Edit native
JavaDoc in the include template, build the plugin, then run
`cargo xtask gen-api`. The generated API page is committed so it
also renders directly on GitHub. Run the generator with `--check` and build the
site with `cargo xtask docs` before submitting documentation changes. See
[the documentation build guide](docs/building.md#documentation-site) for the
documentation build and preview commands.

CodeQL is configured for Rust and Actions. It runs automatically for public
repositories. For a private repository, enable `ENABLE_CODEQL=true` only after
CodeQL entitlement and code scanning are available. Repository access settings,
branch protection and vulnerability-reporting settings are managed separately.
