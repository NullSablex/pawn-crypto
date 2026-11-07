#!/usr/bin/env python3
"""Compile all Pawn sources and optionally test an isolated Linux open.mp server."""

import argparse
import os
import re
import shutil
import socket
import subprocess
import tempfile
import time
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
COMPONENTS = (
    "$CAPI", "Actors", "Checkpoints", "Classes", "Console", "CustomModels",
    "Databases", "Dialogs", "GangZones", "LegacyConfig", "LegacyNetwork", "Menus",
    "NPCs", "Objects", "Pawn", "Pickups", "Recordings", "TextDraws", "TextLabels",
    "Timers", "Variables", "Vehicles",
)


def compile_sources(compiler: Path, stdlib: Path, work: Path) -> dict[str, Path]:
    env = os.environ.copy()
    if os.name == "posix":
        env["LD_LIBRARY_PATH"] = str(compiler.parent) + os.pathsep + env.get("LD_LIBRARY_PATH", "")
    sources = sorted((ROOT / "examples").glob("*.pwn"))
    sources.extend(sorted((ROOT / "tests/pawn").glob("*.pwn")))
    if not sources:
        raise RuntimeError("No Pawn sources found")
    fixtures = {}
    for source in sources:
        output = work / "compiled" / source.relative_to(ROOT).with_suffix(".amx")
        output.parent.mkdir(parents=True, exist_ok=True)
        result = subprocess.run(
            [str(compiler), str(source), "-i" + str(ROOT / "include"),
             "-i" + str(stdlib), "-o" + str(output)], env=env, cwd=ROOT,
            text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60,
        )
        if result.returncode or re.search(r"\b(?:warning|error)\b", result.stdout, re.IGNORECASE):
            raise RuntimeError(f"{source.relative_to(ROOT)} failed:\n{result.stdout}")
        if not output.is_file():
            raise RuntimeError(f"Compiler produced no output for {source}")
        if source.parent == ROOT / "tests/pawn":
            fixtures[source.stem] = output
    print(f"Compiled {len(sources)} Pawn sources without diagnostics.", flush=True)
    return fixtures


def free_udp_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as probe:
        probe.bind(("127.0.0.1", 0))
        return probe.getsockname()[1]


def run_suite(server: Path, plugin: Path, fixtures: dict[str, Path], work: Path,
              mode: str, suite: str, timeout: float) -> None:
    root = work / f"{suite}-{mode}"
    for name in ("components", "gamemodes", "plugins", "scriptfiles", "filterscripts"):
        (root / name).mkdir(parents=True)
    for name in COMPONENTS:
        source = server / "components" / f"{name}.so"
        if source.is_file():
            (root / "components" / source.name).symlink_to(source)
    for required in ("Pawn", "Timers", "Console", "LegacyNetwork"):
        if not (root / "components" / f"{required}.so").is_file():
            raise RuntimeError(f"Missing required standard component: {required}.so")
    destination = "components" if mode == "native" else "plugins"
    shutil.copy2(plugin, root / destination / "pawn_crypto.so")
    shutil.copy2(fixtures[suite], root / "gamemodes/test.amx")
    if suite == "argon2id":
        for name in ("argon2id_other", "argon2id_cancel"):
            shutil.copy2(fixtures[name], root / "filterscripts" / f"{name}.amx")
    config = {
        "announce": False, "max_players": 1,
        "network": {"bind": "127.0.0.1", "port": free_udp_port()},
        "rcon": {"enable": False, "password": "local-test-only"},
        "pawn": {"main_scripts": ["test 1"],
                 "legacy_plugins": ["pawn_crypto"] if mode == "legacy" else []},
    }
    (root / "config.json").write_text(json.dumps(config), encoding="utf-8")
    prefix = "ARGON" if suite == "argon2id" else "CRYPTO"
    completion = re.compile(rf"{prefix}_TEST_DONE: 0 failures, (\d+) checks")
    stdout_path = root / "stdout.log"
    with stdout_path.open("w", encoding="utf-8") as stdout:
        process = subprocess.Popen([str(server / "omp-server")], cwd=root,
                                   stdin=subprocess.DEVNULL, stdout=stdout,
                                   stderr=subprocess.STDOUT)
        try:
            deadline = time.monotonic() + timeout
            while True:
                paths = (root / "log.txt", stdout_path)
                logs = "\n".join(path.read_text(encoding="utf-8", errors="replace")
                                 for path in paths if path.exists())
                if "ARGON_TEST_FAILED" in logs or "CRYPTO_TEST_FAILED" in logs:
                    raise RuntimeError(f"{suite}/{mode} failed; logs: {root}\n{logs[-8000:]}")
                match = completion.search(logs)
                if match:
                    if suite == "argon2id" and "ARGON_OWNER_OK" not in logs:
                        raise RuntimeError(f"Missing AMX ownership check; logs: {root}")
                    if process.poll() not in (None, 0):
                        raise RuntimeError(f"Server exited with {process.returncode}; logs: {root}")
                    print(f"{suite}/{mode}: {match.group(1)} checks passed; logs: {root}", flush=True)
                    return
                if process.poll() is not None or time.monotonic() >= deadline:
                    raise RuntimeError(f"{suite}/{mode} did not complete; logs: {root}\n{logs[-8000:]}")
                time.sleep(0.05)
        finally:
            if process.poll() is None:
                process.terminate()
                try:
                    process.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--compiler", default="pawncc", help="Pawn compiler executable")
    parser.add_argument("--stdlib", required=True, type=Path, help="SA-MP standard include directory")
    parser.add_argument("--server", type=Path, help="Installed Linux open.mp server directory")
    parser.add_argument("--plugin", type=Path, default=ROOT / "dist/pawn_crypto.so")
    parser.add_argument("--timeout", type=float, default=20, help="Seconds per runtime suite")
    args = parser.parse_args()
    compiler = Path(shutil.which(args.compiler) or args.compiler).resolve()
    if not compiler.is_file() or not (args.stdlib / "a_samp.inc").is_file():
        parser.error("Provide an existing compiler and a directory containing a_samp.inc")
    if args.timeout <= 0:
        parser.error("--timeout must be positive")
    if args.server and (not (args.server / "omp-server").is_file() or not args.plugin.is_file()):
        parser.error("Provide an installed server and built plugin (run scripts/build-linux.sh first)")
    work = Path(tempfile.mkdtemp(prefix="pawn-crypto-check-"))
    print(f"Test workspace: {work}", flush=True)
    try:
        fixtures = compile_sources(compiler, args.stdlib.resolve(), work)
        if args.server:
            for mode in ("native", "legacy"):
                for suite in ("crypto", "argon2id"):
                    run_suite(args.server.resolve(), args.plugin.resolve(), fixtures, work,
                              mode, suite, args.timeout)
    except (OSError, RuntimeError, subprocess.SubprocessError) as error:
        print(str(error))
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
