# Installation

## Download

Each [release](https://github.com/NullSablex/pawn-crypto/releases) attaches
the 32-bit (x86) plugin binaries and the Pawn includes:

| File | Use |
| --- | --- |
| `pawn_crypto.so` | Linux GNU x86 plugin. |
| `pawn_crypto.dll` | Windows MSVC x86 plugin. |
| `crypto.inc` / `crypto_omp.inc` | Pawn includes (snake_case or PascalCase). |
| `THIRD-PARTY-LICENSES.html` | Licenses of the dependencies compiled into the plugin. |

`SHA256SUMS` lists their checksums; verify with
`sha256sum -c SHA256SUMS --ignore-missing`. To compile the binaries yourself,
see [building from source](building.md).

Copy `crypto.inc` to your Pawn compiler's include directory, such as
`pawno/include/` or `qawno/include/`. Choose one loading
mode per server; do not load the same binary through two modes.

## SA-MP plugin

Copy the binary to the server's `plugins/` directory. Append it to the existing
`plugins` line in `server.cfg`, preserving other plugin entries.

### Linux

```text
plugins pawn_crypto.so
```

### Windows

```text
plugins pawn_crypto
```

## Native open.mp component

Copy the default binary into the server's `components/` directory. The server
loads its native component entry point automatically. No `pawn.legacy_plugins`
entry is needed for this mode.

A build using the `samp-only` feature does not expose the native component entry
point; load that build through legacy mode instead.

## open.mp legacy plugin

Copy the binary into `plugins/`. Add its base name to `pawn.legacy_plugins` in
`config.json`, preserving the rest of the server configuration:

```json
{
  "pawn": {
    "legacy_plugins": ["pawn_crypto"]
  }
}
```

This is a configuration fragment, not a replacement for your whole file. The
server selects the platform's shared-library extension.

## Pawn naming style

### snake_case

```pawn
#include <a_samp>
#include <crypto>
// crypto_sha256(...)
```

### PascalCase aliases

```pawn
#include <a_samp>
#include <crypto_omp>
// Crypto_Sha256(...)
```

Install `crypto_omp.inc` as well if you choose that naming style. It is a
standalone alternative to `crypto.inc`; include only one per script. Both
styles work with either server. The open.mp Pawn compiler can use the SA-MP
standard includes, so `<open.mp>` is not required.

## Check startup

The plugin logs startup metadata with the `[Crypto]` prefix and writes
`logs/crypto.log`. Compile one of the examples and confirm that its native calls
succeed. A missing-native message usually means the binary was not loaded,
loading failed, or the script's include does not match the loaded binary.
See [troubleshooting](errors.md).
