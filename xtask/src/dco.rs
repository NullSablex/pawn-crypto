use std::process::Command;

use crate::{Result, root};

fn git(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root())
        .output()?;
    if !output.status.success() {
        return Err(format!("git failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn valid_identity(identity: &str) -> bool {
    let Some((name, email)) = identity.trim().rsplit_once(" <") else {
        return false;
    };
    let Some(email) = email.strip_suffix('>') else {
        return false;
    };
    !name.trim().is_empty()
        && email.contains('@')
        && !email
            .chars()
            .any(|ch| ch.is_whitespace() || ch == '<' || ch == '>')
}

fn check(author: &str, signoffs: &str, coauthors: &str) -> Result<()> {
    let signoffs: Vec<_> = signoffs.lines().map(str::trim).collect();
    for identity in std::iter::once(author.trim()).chain(
        coauthors
            .lines()
            .map(str::trim)
            .filter(|identity| !identity.is_empty()),
    ) {
        if !valid_identity(identity) || !signoffs.contains(&identity) {
            return Err("missing or invalid author/co-author Signed-off-by trailer".into());
        }
    }
    Ok(())
}

pub fn run(args: &[String]) -> Result<()> {
    if args.len() != 4 || args[0] != "--base" || args[2] != "--head" {
        return Err("Usage: cargo xtask check-dco --base SHA --head SHA".into());
    }
    let base = &args[1];
    let head = &args[3];
    for sha in [base, head] {
        if sha.len() != 40 || !sha.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("DCO range endpoints must be full 40-character commit SHAs".into());
        }
        git(&["cat-file", "-e", &format!("{sha}^{{commit}}")])?;
    }
    let range = format!("{base}..{head}");
    let commits = git(&["rev-list", "--reverse", &range])?;
    if commits.trim().is_empty() {
        return Err("DCO range contains no commits".into());
    }
    for sha in commits.lines() {
        let author = git(&["show", "-s", "--format=%an <%ae>", sha])?;
        // Git parses the final trailer block; prose mentioning a sign-off does
        // not count as a certification. Do not execute code from commit text.
        let signoffs = git(&[
            "show",
            "-s",
            "--format=%(trailers:key=Signed-off-by,valueonly,unfold=true)",
            sha,
        ])?;
        let coauthors = git(&[
            "show",
            "-s",
            "--format=%(trailers:key=Co-authored-by,valueonly,unfold=true)",
            sha,
        ])?;
        check(&author, &signoffs, &coauthors)
            .map_err(|error| format!("DCO check failed for {sha}: {error}"))?;
    }
    println!(
        "DCO sign-offs verified for {} commits.",
        commits.lines().count()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check;

    const AUTHOR: &str = "Contributor <contributor@example.org>";
    const COAUTHOR: &str = "Coauthor <coauthor@example.org>";

    #[test]
    fn requires_the_authors_own_signoff() {
        assert!(check(AUTHOR, AUTHOR, "").is_ok());
        assert!(check(AUTHOR, &format!("{AUTHOR}\n\n"), "\n").is_ok());
        assert!(check(AUTHOR, "", "").is_err());
        assert!(check(AUTHOR, COAUTHOR, "").is_err());
    }

    #[test]
    fn requires_each_coauthors_signoff() {
        assert!(check(AUTHOR, AUTHOR, COAUTHOR).is_err());
        assert!(check(AUTHOR, &format!("{AUTHOR}\n{COAUTHOR}"), COAUTHOR).is_ok());
    }

    #[test]
    fn refuses_incomplete_identities() {
        for identity in [
            "Contributor",
            " <user@example.org>",
            "Name <>",
            "Name <a b@c>",
        ] {
            assert!(check(identity, identity, "").is_err());
        }
    }
}
