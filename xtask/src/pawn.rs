use crate::{Result, files, read, root};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use regex::Regex;
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    net::UdpSocket,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const COMPONENTS: &[&str] = &[
    "$CAPI",
    "Actors",
    "Checkpoints",
    "Classes",
    "Console",
    "CustomModels",
    "Databases",
    "Dialogs",
    "GangZones",
    "LegacyConfig",
    "LegacyNetwork",
    "Menus",
    "NPCs",
    "Objects",
    "Pawn",
    "Pickups",
    "Recordings",
    "TextDraws",
    "TextLabels",
    "Timers",
    "Variables",
    "Vehicles",
];

struct Options {
    compiler: PathBuf,
    stdlib: PathBuf,
    server: Option<PathBuf>,
    plugin: PathBuf,
    timeout: Duration,
    docs: bool,
}

fn executable(name: &Path) -> Result<PathBuf> {
    if name.is_file() {
        return Ok(name.canonicalize()?);
    }
    for directory in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
        let candidate = directory.join(name);
        if candidate.is_file() {
            return Ok(candidate.canonicalize()?);
        }
        if cfg!(windows) && candidate.with_extension("exe").is_file() {
            return Ok(candidate.with_extension("exe").canonicalize()?);
        }
    }
    Err(format!("executable not found: {}", name.display()).into())
}

fn options(args: &[String]) -> Result<Options> {
    let mut compiler = PathBuf::from("pawncc");
    let mut stdlib = None;
    let mut server = None;
    let mut plugin = root().join("dist/pawn_crypto.so");
    let mut seconds = 20;
    let mut docs = false;
    let mut args = args.iter();
    while let Some(flag) = args.next() {
        if flag == "--docs" {
            docs = true;
            continue;
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--compiler" => compiler = PathBuf::from(value),
            "--stdlib" => stdlib = Some(PathBuf::from(value)),
            "--server" => server = Some(PathBuf::from(value)),
            "--plugin" => plugin = PathBuf::from(value),
            "--timeout" => seconds = value.parse::<u64>()?,
            _ => return Err(format!("unknown option: {flag}").into()),
        }
    }
    let stdlib = stdlib.ok_or("--stdlib is required")?.canonicalize()?;
    if !stdlib.join("a_samp.inc").is_file() || !(1..=3600).contains(&seconds) {
        return Err("provide SA-MP includes and a timeout between 1 and 3600 seconds".into());
    }
    if let Some(path) = &server
        && (!path.join("omp-server").is_file() || !plugin.is_file())
    {
        return Err("provide a Linux open.mp server directory and a built plugin".into());
    }
    Ok(Options {
        compiler: executable(&compiler)?,
        stdlib,
        server,
        plugin,
        timeout: Duration::from_secs(seconds),
        docs,
    })
}

fn compile(options: &Options, source: &Path, output: &Path) -> Result<()> {
    fs::create_dir_all(output.parent().ok_or("output needs a parent")?)?;
    let mut command = Command::new(&options.compiler);
    command
        .arg(source)
        .arg(format!("-i{}", root().join("include").display()))
        .arg(format!("-i{}", options.stdlib.display()))
        .arg(format!("-o{}", output.display()))
        .current_dir(root());
    if cfg!(unix) {
        let mut directories = vec![
            options
                .compiler
                .parent()
                .ok_or("compiler needs a parent")?
                .to_owned(),
        ];
        directories.extend(std::env::split_paths(
            &std::env::var_os("LD_LIBRARY_PATH").unwrap_or_default(),
        ));
        command.env("LD_LIBRARY_PATH", std::env::join_paths(directories)?);
    }
    let result = command.output()?;
    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    if !result.status.success()
        || Regex::new(r"(?i)\b(warning|error)\b")?.is_match(&diagnostics)
        || !output.is_file()
    {
        return Err(format!(
            "{}: compiler failed ({})\n{diagnostics}",
            source.display(),
            result.status
        )
        .into());
    }
    Ok(())
}

fn snippets(options: &Options, work: &Path) -> Result<usize> {
    let mut pages = files(&root().join("docs"), "md")?;
    pages.push(root().join("README.md"));
    let mut count = 0;
    for page in pages {
        let mut code = None;
        for event in Parser::new(&read(&page)?) {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info)))
                    if info.as_ref() == "pawn" =>
                {
                    code = Some(String::new())
                }
                Event::Text(text) => {
                    if let Some(code) = &mut code {
                        code.push_str(&text);
                    }
                }
                Event::End(TagEnd::CodeBlock) => {
                    if let Some(code) = code.take().filter(|code| {
                        code.contains("#include <a_samp>")
                            && code.contains("public OnFilterScriptInit()")
                    }) {
                        if !code.is_ascii() {
                            return Err(
                                format!("{}: non-ASCII Pawn example", page.display()).into()
                            );
                        }
                        count += 1;
                        let path = work.join(format!("docs/{count}.pwn"));
                        fs::create_dir_all(path.parent().ok_or("snippet needs a parent")?)?;
                        fs::write(&path, code)?;
                        compile(options, &path, &path.with_extension("amx"))?;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(count)
}

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        if !matches!(self.0.try_wait(), Ok(Some(_))) {
            if let Some(stdin) = self.0.stdin.as_mut() {
                let _ = stdin.write_all(b"exit\n");
            }
            let deadline = Instant::now() + Duration::from_millis(300);
            while matches!(self.0.try_wait(), Ok(None)) && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(20));
            }
            if !matches!(self.0.try_wait(), Ok(Some(_))) {
                let _ = self.0.kill();
            }
            let _ = self.0.wait();
        }
    }
}

fn runtime(
    options: &Options,
    fixtures: &BTreeMap<String, PathBuf>,
    work: &Path,
    mode: &str,
    suite: &str,
) -> Result<()> {
    let source = options
        .server
        .as_ref()
        .ok_or("server is required")?
        .canonicalize()?;
    let directory = work.join(format!("{suite}-{mode}"));
    for name in [
        "components",
        "gamemodes",
        "plugins",
        "scriptfiles",
        "filterscripts",
    ] {
        fs::create_dir_all(directory.join(name))?;
    }
    for name in COMPONENTS {
        let component = source.join(format!("components/{name}.so"));
        if component.is_file() {
            #[cfg(unix)]
            std::os::unix::fs::symlink(
                &component,
                directory.join(format!("components/{name}.so")),
            )?;
            #[cfg(not(unix))]
            fs::copy(&component, directory.join(format!("components/{name}.so")))?;
        }
    }
    for name in ["Pawn", "Timers", "Console", "LegacyNetwork"] {
        if !directory.join(format!("components/{name}.so")).is_file() {
            return Err(format!("missing standard component: {name}.so").into());
        }
    }
    let destination = if mode == "native" {
        "components"
    } else {
        "plugins"
    };
    fs::copy(
        &options.plugin,
        directory.join(format!("{destination}/pawn_crypto.so")),
    )?;
    fs::copy(
        fixtures.get(suite).ok_or("missing gamemode fixture")?,
        directory.join("gamemodes/test.amx"),
    )?;
    if suite == "argon2id" {
        for name in ["argon2id_other", "argon2id_cancel"] {
            fs::copy(
                fixtures.get(name).ok_or("missing filterscript fixture")?,
                directory.join(format!("filterscripts/{name}.amx")),
            )?;
        }
    }
    let port = UdpSocket::bind(("127.0.0.1", 0))?.local_addr()?.port();
    let plugins = if mode == "legacy" {
        vec!["pawn_crypto"]
    } else {
        Vec::new()
    };
    let config = serde_json::json!({"announce":false,"max_players":1,"network":{"bind":"127.0.0.1","port":port},"rcon":{"enable":false,"password":"local-test-only"},"pawn":{"main_scripts":["test 1"],"legacy_plugins":plugins}});
    fs::write(directory.join("config.json"), serde_json::to_vec(&config)?)?;
    let stdout_path = directory.join("stdout.log");
    let stdout = fs::File::create(&stdout_path)?;
    let mut server = Server(
        Command::new(source.join("omp-server"))
            .current_dir(&directory)
            .stdin(Stdio::piped())
            .stdout(stdout.try_clone()?)
            .stderr(stdout)
            .spawn()?,
    );
    let prefix = if suite == "argon2id" {
        "ARGON"
    } else {
        "CRYPTO"
    };
    let completion = Regex::new(&format!(r"{prefix}_TEST_DONE: 0 failures, (\d+) checks"))?;
    let deadline = Instant::now() + options.timeout;
    loop {
        let mut logs = String::new();
        for path in [directory.join("log.txt"), stdout_path.clone()] {
            if path.is_file() {
                logs.push_str(&String::from_utf8_lossy(&fs::read(path)?));
            }
        }
        if logs.contains("ARGON_TEST_FAILED") || logs.contains("CRYPTO_TEST_FAILED") {
            return Err(format!(
                "{suite}/{mode} failed; logs: {}\n{logs}",
                directory.display()
            )
            .into());
        }
        let status = server.0.try_wait()?;
        if let Some(done) = completion.captures(&logs) {
            if suite == "argon2id" && !logs.contains("ARGON_OWNER_OK") {
                return Err("missing AMX ownership check".into());
            }
            if status.is_some_and(|status| !status.success()) {
                return Err("server exited abnormally".into());
            }
            println!(
                "{suite}/{mode}: {} checks passed; logs: {}",
                &done[1],
                directory.display()
            );
            return Ok(());
        }
        if status.is_some() || Instant::now() >= deadline {
            return Err(format!(
                "{suite}/{mode} did not complete; logs: {}\n{logs}",
                directory.display()
            )
            .into());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

pub fn run(args: &[String]) -> Result<()> {
    let options = options(args)?;
    let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let work =
        std::env::temp_dir().join(format!("pawn-crypto-check-{}-{unique}", std::process::id()));
    fs::create_dir(&work)?;
    println!("Test workspace: {}", work.display());
    let mut sources = files(&root().join("examples"), "pwn")?;
    sources.extend(files(&root().join("tests/pawn"), "pwn")?);
    let mut fixtures = BTreeMap::new();
    for source in &sources {
        let output = work
            .join("compiled")
            .join(source.strip_prefix(root())?)
            .with_extension("amx");
        compile(&options, source, &output)?;
        if source.parent() == Some(root().join("tests/pawn").as_path()) {
            fixtures.insert(
                source
                    .file_stem()
                    .ok_or("missing stem")?
                    .to_string_lossy()
                    .into_owned(),
                output,
            );
        }
    }
    println!(
        "Compiled {} Pawn sources without diagnostics.",
        sources.len()
    );
    if options.docs {
        println!(
            "Compiled {} documentation examples without diagnostics.",
            snippets(&options, &work)?
        );
    }
    if options.server.is_some() {
        for mode in ["native", "legacy"] {
            for suite in ["crypto", "argon2id"] {
                runtime(&options, &fixtures, &work, mode, suite)?;
            }
        }
    }
    Ok(())
}
