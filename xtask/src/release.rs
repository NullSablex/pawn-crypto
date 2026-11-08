use std::path::Path;

use crate::{Result, read, root};

fn prepare(version: &str, tag: &str, channel: &str, changelog: &str) -> Result<(bool, String)> {
    if tag != format!("v{version}") {
        return Err("release tag must equal v followed by the Cargo package version".into());
    }
    let prefix = format!("## [{version}]");
    let entries: Vec<_> = changelog
        .lines()
        .enumerate()
        .filter(|(_, line)| line.starts_with(&prefix))
        .collect();
    let [(index, header)] = entries.as_slice() else {
        return Err("changelog must contain exactly one entry for the package version".into());
    };
    let suffix = header.strip_prefix(&prefix).unwrap();
    let prerelease = suffix == " - Pre-release";
    let dated = regex::Regex::new(r"^ - [0-9]{4}/[0-9]{2}/[0-9]{2}$")?;
    if !prerelease && !suffix.is_empty() && !dated.is_match(suffix) {
        return Err("release header must be plain, dated YYYY/MM/DD, or marked Pre-release".into());
    }
    if !prerelease && version.contains('-') {
        return Err("a version with a prerelease suffix cannot be published as stable".into());
    }
    match channel {
        "auto" => (),
        "prerelease" if prerelease => (),
        "release" if !prerelease => (),
        "prerelease" | "release" => {
            return Err("requested release channel disagrees with the changelog".into());
        }
        _ => return Err("channel must be auto, prerelease or release".into()),
    }
    let body = changelog
        .lines()
        .skip(index + 1)
        .take_while(|line| !line.starts_with("## "))
        .collect::<Vec<_>>()
        .join("\n");
    if body.trim().is_empty() {
        return Err("release changelog entry must not be empty".into());
    }
    Ok((prerelease, body.trim().to_owned()))
}

pub fn run(args: &[String]) -> Result<()> {
    if args.len() != 6 || args[0] != "--tag" || args[2] != "--channel" || args[4] != "--out" {
        return Err("Usage: cargo xtask prepare-release --tag TAG --channel auto|prerelease|release --out DIR".into());
    }
    let tag = &args[1];
    let manifest: toml::Value = toml::from_str(&read(&root().join("Cargo.toml"))?)?;
    let package = &manifest["package"];
    let version = package["version"]
        .as_str()
        .ok_or("missing package version")?;
    let repository = package["repository"]
        .as_str()
        .ok_or("missing package repository")?;
    let (prerelease, body) = prepare(version, tag, &args[3], &read(&root().join("CHANGELOG.md"))?)?;
    let notes = format!(
        "{body}\n\n### Downloads\n\n\
         - `pawn_crypto.so`: Linux GNU x86.\n\
         - `pawn_crypto.dll`: Windows MSVC x86.\n\
         - `crypto.inc` and `crypto_omp.inc`: alternative Pawn naming styles.\n\
         - Platform archives: binary, includes, documentation, examples and licenses.\n\
         - `licenses.json` and `THIRD-PARTY-LICENSES.html`: dependency inventories.\n\
         - `SHA256SUMS`: checksums of the downloadable assets.\n\n\
         [Installation guide]({repository}/blob/{tag}/docs/installation.md).\n"
    );
    let output = Path::new(&args[5]);
    std::fs::create_dir_all(output)?;
    std::fs::write(output.join("notes.md"), notes)?;
    std::fs::write(
        output.join("metadata.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "tag": tag, "version": version, "prerelease": prerelease,
        }))?,
    )?;
    println!(
        "Prepared {tag} (prerelease={prerelease}); notes: {}",
        output.join("notes.md").display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::prepare;

    #[test]
    fn extracts_only_the_requested_entry_and_its_channel() {
        let changelog = "# Changelog\n\n## [0.1.0] - Pre-release\n\n### Added\n\n- Hashing.\n\n## [0.0.1]\n\n- Older.\n";
        let (prerelease, notes) = prepare("0.1.0", "v0.1.0", "auto", changelog).unwrap();
        assert!(prerelease);
        assert_eq!(notes, "### Added\n\n- Hashing.");
        assert!(prepare("0.1.0", "v0.1.0", "release", changelog).is_err());
    }

    #[test]
    fn stable_and_semver_prerelease_versions_are_distinct() {
        let stable = "## [1.0.0] - 2026/11/07\n\n- Stable.\n";
        assert!(!prepare("1.0.0", "v1.0.0", "release", stable).unwrap().0);
        assert!(prepare("1.0.0", "v1.0.0", "prerelease", stable).is_err());
        let invalid = "## [1.0.0-rc.1]\n\n- Candidate.\n";
        assert!(prepare("1.0.0-rc.1", "v1.0.0-rc.1", "auto", invalid).is_err());
        let candidate = "## [1.0.0-rc.1] - Pre-release\n\n- Candidate.\n";
        assert!(
            prepare("1.0.0-rc.1", "v1.0.0-rc.1", "auto", candidate)
                .unwrap()
                .0
        );
    }

    #[test]
    fn refuses_mismatched_tags_missing_duplicate_or_empty_entries() {
        assert!(prepare("0.1.0", "v0.2.0", "auto", "").is_err());
        assert!(prepare("0.1.0", "v0.1.0", "auto", "# Changelog").is_err());
        assert!(prepare("0.1.0", "v0.1.0", "auto", "## [0.1.0]\n").is_err());
        assert!(
            prepare(
                "0.1.0",
                "v0.1.0",
                "auto",
                "## [0.1.0]\n- One\n## [0.1.0]\n- Two"
            )
            .is_err()
        );
    }
}
