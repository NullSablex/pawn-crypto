use crate::{Result, files, read, root};
use regex::Regex;
use std::collections::BTreeSet;

fn captures(text: &str, pattern: &str) -> Result<BTreeSet<String>> {
    Ok(Regex::new(pattern)?
        .captures_iter(text)
        .map(|capture| capture[1].to_owned())
        .collect())
}

pub fn check() -> Result<()> {
    let mut paths = vec![
        root().join("include/crypto.inc.in"),
        root().join("include/crypto.inc"),
        root().join("include/crypto_omp.inc"),
    ];
    paths.extend(files(&root().join("examples"), "pwn")?);
    paths.extend(files(&root().join("tests/pawn"), "pwn")?);
    for path in paths {
        let data = std::fs::read(&path)?;
        if let Some(offset) = data
            .iter()
            .position(|&byte| byte > 126 || (byte < 32 && byte != b'\t' && byte != b'\n'))
        {
            let line = data[..offset].iter().filter(|&&byte| byte == b'\n').count() + 1;
            let column = offset
                - data[..offset]
                    .iter()
                    .rposition(|&byte| byte == b'\n')
                    .map_or(0, |at| at + 1)
                + 1;
            return Err(format!(
                "{}:{line}:{column}: expected ASCII, LF, no BOM or stray control bytes",
                path.display()
            )
            .into());
        }
    }
    let manifest: toml::Table = read(&root().join("Cargo.toml"))?.parse()?;
    let version = manifest
        .get("package")
        .and_then(|package| package.get("version"))
        .and_then(toml::Value::as_str)
        .ok_or("missing package version")?;
    let base = read(&root().join("include/crypto.inc"))?;
    let styled = read(&root().join("include/crypto_omp.inc"))?;
    let template = read(&root().join("include/crypto.inc.in"))?;
    if base != template.replace("{{VERSION}}", version) {
        return Err("crypto.inc is stale; run cargo build".into());
    }
    let registered = captures(
        &read(&root().join("src/lib.rs"))?,
        r"CryptoPlugin::(crypto_\w+)",
    )?;
    let declared = captures(&base, r"(?m)^native (?:\w+:)?(crypto_\w+)\(")?;
    let mut annotated = BTreeSet::new();
    for path in files(&root().join("src/natives"), "rs")? {
        annotated.extend(captures(
            &read(&path)?,
            r#"#\[native\(name = "(crypto_\w+)""#,
        )?);
    }
    if registered.is_empty() || registered != declared || registered != annotated {
        return Err(format!("native declarations differ: registered={registered:?}, declared={declared:?}, annotated={annotated:?}").into());
    }
    let alias_pattern = Regex::new(r"(?m)^native (?:\w+:)?(\w+)\(.*\) = (crypto_\w+);$")?;
    let aliases: BTreeSet<_> = alias_pattern
        .captures_iter(&styled)
        .map(|cap| (cap[1].to_owned(), cap[2].to_owned()))
        .collect();
    let expected: BTreeSet<_> = declared
        .iter()
        .map(|name| {
            let mut words = name.split('_');
            let prefix = capitalize(words.next().unwrap_or_default());
            (
                format!("{prefix}_{}", words.map(capitalize).collect::<String>()),
                name.clone(),
            )
        })
        .collect();
    if aliases != expected
        || alias_pattern.captures_iter(&styled).count() != expected.len()
        || !styled.contains(&format!("#define CRYPTO_VERSION \"{version}\""))
    {
        return Err("crypto_omp.inc aliases or version are stale; run cargo build".into());
    }
    println!("Pawn encoding, generated version, native declarations and aliases are consistent.");
    Ok(())
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_ascii_uppercase().to_string() + chars.as_str()
    })
}
