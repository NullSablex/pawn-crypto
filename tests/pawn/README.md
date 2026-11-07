# Pawn runtime fixtures

These fixtures exercise the actual AMX native boundary. Use
`scripts/check-pawn.py --compiler ... --stdlib ... --server ...` to compile them
and run isolated Linux open.mp instances in native and legacy modes.

- `crypto.pwn` checks all SHA-2 and HMAC natives, buffer handling, raw bytes,
  packed strings, tampered tags and verification failures.
- `argon2id.pwn` checks asynchronous submission, queue capacity, PHC results,
  verification, callback re-entry and script ownership.
- `argon2id_other.pwn` supplies a second AMX with a colliding callback name.
- `argon2id_cancel.pwn` exercises cancellation across unload and reload.

The runner checks completion and failure markers and reports log locations.
No fixture needs real players, passwords, keys or a database.
