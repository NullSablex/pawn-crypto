#!/usr/bin/env python3
"""Check committed Pawn sources, generated includes and native declarations."""

import argparse
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def check() -> list[str]:
    problems = []
    paths = sorted((ROOT / "include").glob("*"))
    for directory in ("examples", "tests/pawn"):
        paths.extend(sorted((ROOT / directory).rglob("*.pwn")))
    for path in paths:
        data = path.read_bytes()
        bad = next((i for i, byte in enumerate(data)
                    if byte > 126 or (byte < 32 and byte not in (9, 10))), None)
        if bad is not None:
            line = data.count(b"\n", 0, bad) + 1
            column = bad - data.rfind(b"\n", 0, bad)
            problems.append(f"{path.relative_to(ROOT)}:{line}:{column}: "
                            "expected ASCII without BOM, LF endings and no stray control bytes")

    manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    template = (ROOT / "include/crypto.inc.in").read_text(encoding="utf-8")
    base = (ROOT / "include/crypto.inc").read_text(encoding="utf-8")
    styled = (ROOT / "include/crypto_omp.inc").read_text(encoding="utf-8")
    if base != template.replace("{{VERSION}}", manifest["package"]["version"]):
        problems.append("include/crypto.inc is stale; run cargo build")

    registered = set(re.findall(r"CryptoPlugin::(crypto_\w+)",
                                (ROOT / "src/lib.rs").read_text(encoding="utf-8")))
    declared = set(re.findall(r"^native (?:\w+:)?(crypto_\w+)\(", base, re.MULTILINE))
    annotated = set()
    for path in (ROOT / "src/natives").glob("*.rs"):
        annotated.update(re.findall(r'#\[native\(name = "(crypto_\w+)"',
                                    path.read_text(encoding="utf-8")))
    if not registered or registered != declared or registered != annotated:
        problems.append("Rust registrations, explicit native names and include declarations differ: "
                        f"registered={sorted(registered)}, declared={sorted(declared)}, "
                        f"annotated={sorted(annotated)}")

    aliases = re.findall(r"^native (?:\w+:)?(\w+)\(.*\) = (crypto_\w+);$",
                         styled, re.MULTILINE)
    expected_aliases = set()
    for name in declared:
        prefix, *words = name.split("_")
        expected_aliases.add((prefix.capitalize() + "_" +
                             "".join(word[0].upper() + word[1:] for word in words), name))
    if len(aliases) != len(expected_aliases) or set(aliases) != expected_aliases:
        problems.append("include/crypto_omp.inc aliases differ from the base API; run cargo build")
    if f'#define CRYPTO_VERSION "{manifest["package"]["version"]}"' not in styled:
        problems.append("include/crypto_omp.inc version is stale; run cargo build")
    return problems


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--github", action="store_true", help="emit GitHub Actions annotations")
    args = parser.parse_args()
    try:
        problems = check()
    except (OSError, ValueError) as error:
        problems = [str(error)]
    for problem in problems:
        print(("::error::" if args.github else "") + problem, file=sys.stderr)
    if problems:
        return 1
    print("Pawn encoding, generated version, native declarations and aliases are consistent.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
