use std::process::Command;

pub fn terminal(program: &str, args: &[&str]) {
    let status = match Command::new(program).args(args).status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: cannot run `{program}`: {e}");
            std::process::exit(1);
        }
    };

    if !status.success() {
        eprintln!("Error: `{program} {args:?}` failed");
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

    terminal("git", &["clone", "--no-checkout", "--filter=blob:none", &url, &tmp]);
    terminal("git", &["-C", &tmp, "sparse-checkout", "init", "--cone"]);
    terminal("git", &["-C", &tmp, "sparse-checkout", "set", folder]);
    terminal("git", &["-C", &tmp, "checkout"]);

    if let Err(e) = std::fs::rename(format!("{tmp}/{folder}"), dest) {
        eprintln!("Error: cannot rename `{tmp}/{folder}` -> `{dest}`: {e}");
        std::process::exit(1);
    }

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