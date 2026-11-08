mod dco;
mod docs;
mod pawn;
mod release;
mod sources;

use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask has a parent")
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()).into())
}

fn files(directory: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    for entry in std::fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            result.extend(files(&path, extension)?);
        } else if path.extension().is_some_and(|value| value == extension) {
            result.push(path);
        }
    }
    result.sort();
    Ok(result)
}

fn main() -> std::process::ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let github = args.iter().any(|arg| arg == "--github");
    let result = match args.first().map(String::as_str) {
        Some("check-pawn") if args.len() == 1 || args[1..] == ["--github"] => sources::check(),
        Some("gen-api") if args.len() == 1 || args[1..] == ["--check"] => docs::generate(args.len() == 2),
        Some("docs") if args.len() == 1 => docs::build(),
        Some("pawn") => pawn::run(&args[1..]),
        Some("check-dco") => dco::run(&args[1..]),
        Some("prepare-release") => release::run(&args[1..]),
        _ => Err("Usage: cargo xtask {check-pawn [--github]|check-dco --base SHA --head SHA|prepare-release --tag TAG --channel auto|prerelease|release --out DIR|gen-api [--check]|docs|pawn --compiler PATH --stdlib DIR [--server DIR] [--plugin PATH] [--timeout SECONDS] [--docs]}".into()),
    };
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            if github {
                eprintln!(
                    "::error::{}",
                    error
                        .to_string()
                        .replace('%', "%25")
                        .replace('\n', "%0A")
                        .replace('\r', "%0D")
                );
            } else {
                eprintln!("{error}");
            }
            std::process::ExitCode::FAILURE
        }
    }
}
