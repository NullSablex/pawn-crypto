use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

const TAG: &str = "v0.1.0-alpha.1";
const COMMIT: &str = "0123456789012345678901234567890123456789";
const ASSETS: &[&str] = &[
    "pawn_crypto.so",
    "pawn_crypto.dll",
    "crypto.inc",
    "crypto_omp.inc",
    "LICENSE",
    "licenses.json",
    "THIRD-PARTY-LICENSES.html",
    "SHA256SUMS",
    "pawn-crypto-v0.1.0-alpha.1-linux-x86.tar.gz",
    "pawn-crypto-v0.1.0-alpha.1-windows-x86.zip",
];

struct Fixture(PathBuf);

impl Fixture {
    fn new(draft: bool, scenario: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "pawn-crypto-publish-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(path.join("bin")).unwrap();
        fs::create_dir(path.join("dist")).unwrap();
        fs::create_dir(path.join("release-notes")).unwrap();
        fs::write(path.join("release-notes/notes.md"), "Curated notes.\n").unwrap();
        for name in ASSETS {
            fs::write(path.join("dist").join(name), format!("fixture: {name}\n")).unwrap();
        }
        fs::write(path.join("scenario"), scenario).unwrap();
        fs::write(
            path.join("tag.json"),
            serde_json::json!({"object": {"type": "commit", "sha": COMMIT}}).to_string(),
        )
        .unwrap();
        fs::write(
            path.join("release.json"),
            serde_json::json!({
                "tag_name": TAG, "draft": draft, "prerelease": true, "body": "", "assets": [],
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            path.join("bin/git"),
            "#!/usr/bin/env bash\nprintf '%s\\n' \"$RELEASE_COMMIT\"\n",
        )
        .unwrap();
        fs::write(
            path.join("bin/gh"),
            r#"#!/usr/bin/env bash
set -eu
printf '%s\n' "$*" >> "$TEST_DIR/calls"
case "$*" in
  'api repos/test/project/git/ref/tags/'*) cat "$TEST_DIR/tag.json" ;;
  'api repos/test/project/git/tags/'*) cat "$TEST_DIR/peeled.json" ;;
  'api repos/test/project/releases/tags/'*)
    scenario=$(cat "$TEST_DIR/scenario")
    if [ "$scenario" = forbidden ]; then
      echo 'gh: Resource not accessible (HTTP 403)' >&2; exit 1
    fi
    if [ "$scenario" = missing ] && [ ! -f "$TEST_DIR/created" ]; then
      echo 'gh: Not Found (HTTP 404)' >&2; exit 1
    fi
    cat "$TEST_DIR/release.json" ;;
  'release create '*) touch "$TEST_DIR/created" ;;
  'release upload '*|'release edit '*) ;;
  *) echo "Unexpected gh call: $*" >&2; exit 1 ;;
esac
"#,
        )
        .unwrap();
        for name in ["gh", "git"] {
            fs::set_permissions(
                path.join("bin").join(name),
                fs::Permissions::from_mode(0o755),
            )
            .unwrap();
        }
        let workflow =
            fs::read_to_string(crate::root().join(".github/workflows/release.yml")).unwrap();
        let (_, step) = workflow
            .split_once("      - name: Publish approved tag with curated notes\n")
            .unwrap();
        let (_, script) = step.split_once("        run: |\n").unwrap();
        let script = script
            .lines()
            .map(|line| line.strip_prefix("          ").unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path.join("publish.sh"), script).unwrap();
        Self(path)
    }

    fn metadata(&self, update: impl FnOnce(&mut serde_json::Value)) {
        let path = self.0.join("release.json");
        let mut value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        update(&mut value);
        fs::write(path, value.to_string()).unwrap();
    }

    fn asset(&self, name: &str) -> serde_json::Value {
        let result = Command::new("sha256sum")
            .arg(self.0.join("dist").join(name))
            .output()
            .unwrap();
        assert!(result.status.success());
        let text = String::from_utf8(result.stdout).unwrap();
        let digest = text.split_whitespace().next().unwrap();
        serde_json::json!({"name": name, "digest": format!("sha256:{digest}")})
    }

    fn run(&self) -> Output {
        Command::new("bash")
            .arg("publish.sh")
            .current_dir(&self.0)
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    self.0.join("bin").display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .env("TEST_DIR", &self.0)
            .env("GITHUB_REPOSITORY", "test/project")
            .env("RELEASE_TAG", TAG)
            .env("RELEASE_PRERELEASE", "true")
            .env("RELEASE_COMMIT", COMMIT)
            .output()
            .unwrap()
    }

    fn calls(&self) -> String {
        fs::read_to_string(self.0.join("calls")).unwrap_or_default()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn publication_creates_missing_release_and_finishes_drafts() {
    for scenario in ["missing", "existing"] {
        let fixture = Fixture::new(true, scenario);
        let result = fixture.run();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            fixture.calls().contains("release create"),
            scenario == "missing"
        );
        assert!(fixture.calls().contains("--draft=false"));
    }
}

#[test]
fn publication_completes_a_release_published_from_github() {
    let fixture = Fixture::new(false, "existing");
    let result = fixture.run();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(fixture.calls().contains("release upload"));
    assert!(fixture.calls().contains("--notes-file"));
    assert!(!fixture.calls().contains("--clobber"));
    assert!(!fixture.calls().contains("--draft=false"));
}

#[test]
fn publication_preserves_existing_assets_and_notes() {
    let fixture = Fixture::new(false, "existing");
    let asset = fixture.asset("pawn_crypto.so");
    fixture.metadata(|data| {
        data["assets"] = serde_json::json!([asset]);
        data["body"] = "Manual notes.".into();
    });
    assert!(fixture.run().status.success());
    let calls = fixture.calls();
    let upload = calls
        .lines()
        .find(|line| line.starts_with("release upload"))
        .unwrap();
    assert!(!upload.contains("dist/pawn_crypto.so"));
    assert!(!calls.contains("release edit"));
}

#[test]
fn publication_is_a_noop_when_all_published_assets_match() {
    let fixture = Fixture::new(false, "existing");
    let assets: Vec<_> = ASSETS.iter().map(|name| fixture.asset(name)).collect();
    fixture.metadata(|data| {
        data["assets"] = assets.into();
        data["body"] = "Manual notes.".into();
    });
    assert!(fixture.run().status.success());
    assert!(!fixture.calls().contains("release upload"));
    assert!(!fixture.calls().contains("release edit"));
}

#[test]
fn publication_refuses_conflicting_assets_and_channel_mismatches() {
    for conflict in [true, false] {
        let fixture = Fixture::new(false, "existing");
        fixture.metadata(|data| {
            if conflict {
                data["assets"] =
                    serde_json::json!([{"name": "pawn_crypto.so", "digest": "sha256:different"}]);
            } else {
                data["prerelease"] = false.into();
            }
        });
        let result = fixture.run();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("::error::"));
        assert!(!fixture.calls().contains("release upload"));
        assert!(!fixture.calls().contains("release edit"));
    }
}

#[test]
fn publication_does_not_treat_api_failures_as_missing_releases() {
    let fixture = Fixture::new(false, "forbidden");
    assert!(!fixture.run().status.success());
    assert!(!fixture.calls().contains("release create"));
}

#[test]
fn publication_checks_annotated_and_moved_tags() {
    let fixture = Fixture::new(false, "existing");
    fs::write(
        fixture.0.join("peeled.json"),
        fs::read(fixture.0.join("tag.json")).unwrap(),
    )
    .unwrap();
    fs::write(
        fixture.0.join("tag.json"),
        serde_json::json!({"object": {"type": "tag", "sha": "annotated"}}).to_string(),
    )
    .unwrap();
    assert!(fixture.run().status.success());
    let moved = Fixture::new(false, "existing");
    fs::write(
        moved.0.join("tag.json"),
        serde_json::json!({"object": {"type": "commit", "sha": "different"}}).to_string(),
    )
    .unwrap();
    assert!(!moved.run().status.success());
    assert!(!moved.calls().contains("release upload"));
    assert!(!moved.calls().contains("release create"));
}
