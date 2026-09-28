use std::process::{Command, Stdio};
use cliclack::{log};
pub fn terminal(program: &str, args: &[&str]) {
    let status = match Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            log::error(format!("Error: cannot run `{program}`: {e}")).unwrap();
            std::process::exit(1);
        }
    };

    if !status.status.success() {
        let stdout = String::from_utf8_lossy(&status.stdout);
        let stderr = String::from_utf8_lossy(&status.stderr);

        log::error(format!("Error: `{program} {args:?}` failed:")).unwrap();

        if !stdout.is_empty() { log::error(format!("stdout:\n{stdout}")).unwrap(); }
        if !stderr.is_empty() { log::error(format!("stderr:\n{stderr}")).unwrap(); }
        std::process::exit(1);
    }
}
/*
 * copy folder from repo to dest.
 *    @example: git_folder("Secure-Your-Soul", "SoulsCLI", "templates/website", "website");
 */
pub fn git_folder(owner: &str, repo: &str, folder: &str, dest: &str) {
    let url = format!("https://github.com/{owner}/{repo}.git");
    let tmp = format!("/tmp/{repo}-clone");
    // Clear tmp/SoulsCLI-clone path
    let _ = std::fs::remove_dir_all(&tmp);
    terminal("git", &["clone", "--no-checkout", "--filter=blob:none", &url, &tmp]);
    terminal("git", &["-C", &tmp, "sparse-checkout", "init", "--cone"]);
    terminal("git", &["-C", &tmp, "sparse-checkout", "set", folder]);
    terminal("git", &["-C", &tmp, "checkout"]);

    let src = format!("{tmp}/{folder}");

    // Przenieś folder – jak rename zawiedzie (EXDEV), skopiuj i usuń tmp
    if std::fs::rename(&src, dest).is_err() {
        // Kopiuj folder rekurencyjnie – wszystko inline
        let mut stack = vec![(src.clone(), dest.to_string())];
        while let Some((s, d)) = stack.pop() {
            std::fs::create_dir_all(&d).unwrap();
            for entry in std::fs::read_dir(&s).unwrap() {
                let entry = entry.unwrap();
                let target = format!("{d}/{}", entry.file_name().to_string_lossy());
                if entry.path().is_dir() {
                    stack.push((entry.path().to_string_lossy().to_string(), target));
                } else {
                    std::fs::copy(entry.path(), &target).unwrap();
                }
            }
        }
    }
    // Clear tmp/SoulsCLI-clone path
    let _ = std::fs::remove_dir_all(&tmp);
}
/// copy files from repo to dest.
/// @example: git_files("Secure-Your-Soul","SoulsCLI",&["Cargo.toml", "README.md"],"output");
#[allow(dead_code)]
pub fn git_files(owner: &str, repo: &str, files: &[&str], dest: &str) {
    let url = format!("https://github.com/{owner}/{repo}.git");
    let tmp = format!("/tmp/{repo}-clone");

    terminal("git", &["clone", "--depth=1", "--filter=blob:none", "--sparse", &url, &tmp]);

    // sparse-checkout set przyjmuje listę ścieżek
    let mut args = vec!["-C", &tmp, "sparse-checkout", "set"];
    args.extend(files);
    terminal("git", &args);

    // Przenieś każdy plik na docelowe miejsce
    for file in files {
        let src = format!("{tmp}/{file}");
        let dst = std::path::Path::new(dest).join(file);

        if let Some(parent) = dst.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("Error: cannot create dir `{}`: {e}", parent.display());
                std::process::exit(1);
            }
        }

        if let Err(e) = std::fs::rename(&src, &dst) {
            eprintln!("Error: cannot rename `{src}` -> `{}`: {e}", dst.display());
            std::process::exit(1);
        }
    }

    let _ = std::fs::remove_dir_all(&tmp);
}
pub fn is_installed(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
pub fn open_browser(url: &str) {
    let result = if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(url).status()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(url).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", url]).status()
    } else {
        log::error(format!("Unsupported OS")).unwrap();
        return;
    };

    if let Err(e) = result {
        log::error(format!("Cannot open browser: {e}")).unwrap();
    }
}
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}