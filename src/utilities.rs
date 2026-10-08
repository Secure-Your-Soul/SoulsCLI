use std::process::{Command, Stdio};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn terminal(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program).args(args).status()?;
    if !status.success() {
        return Err(format!("`{program} {args:?}` failed").into());
    }
    Ok(())
}
pub fn terminal_quiet(program: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(program).args(args).output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut msg = format!("`{program} {args:?}` failed (status {})", output.status);
        if !stderr.trim().is_empty() {
            msg.push_str("\n--- stderr ---\n");
            msg.push_str(stderr.trim_end());
        }
        if !stdout.trim().is_empty() {
            msg.push_str("\n--- stdout ---\n");
            msg.push_str(stdout.trim_end());
        }
        return Err(msg.into());
    }
    Ok(())
}
pub fn git_paths(owner: &str, repo: &str, paths: &[&str], dest: &str) -> Result<()> {
    assert!(!paths.is_empty(), "paths must not be empty");

    let url = format!("https://github.com/{owner}/{repo}.git");
    let tmp = format!(".{repo}-clone");

    let _ = std::fs::remove_dir_all(dest);
    let _ = std::fs::remove_dir_all(&tmp);

    terminal_quiet(
        "git",
        &[
            "clone",
            "--quiet",
            "--depth=1",
            "--filter=blob:none",
            "--no-checkout",
            &url,
            &tmp,
        ],
    )?;

    let mut args = vec!["-C", &tmp, "sparse-checkout", "set", "--no-cone"];
    args.extend_from_slice(paths);
    terminal_quiet("git", &args)?;

    terminal_quiet("git", &["-C", &tmp, "checkout", "--quiet"])?;

    std::fs::create_dir_all(dest)?;
    for p in paths {
        let src = std::path::Path::new(&tmp).join(p);
        for entry in std::fs::read_dir(&src)? {
            let entry = entry?;
            let target = std::path::Path::new(dest).join(entry.file_name());
            if std::fs::rename(entry.path(), &target).is_err() {
                copy_tree(&entry.path(), &target)?;
            }
        }
    }

    let _ = std::fs::remove_dir_all(&tmp);
    Ok(())
}

fn copy_tree(src: &std::path::Path, dst: &std::path::Path) -> Result<()> {
    let mut stack = vec![(src.to_path_buf(), dst.to_path_buf())];
    while let Some((s, d)) = stack.pop() {
        std::fs::create_dir_all(&d)?;
        for entry in std::fs::read_dir(&s)? {
            let entry = entry?;
            let target = d.join(entry.file_name());
            if entry.path().is_dir() {
                stack.push((entry.path(), target));
            } else {
                std::fs::copy(entry.path(), &target)?;
            }
        }
    }
    Ok(())
}
/// Pobiera wskazane pliki/foldery z repo i umieszcza je w `dest`.
/// Nazwa na dysku = nazwa z repo (bez ścieżki).
/// Nie usuwa niczego innego z `dest`.
pub fn git_update(owner: &str, repo: &str, dest: &str, mapping: &[(&str, &str)]) -> Result<()> {
    assert!(!mapping.is_empty(), "mapping must not be empty");

    let url = format!("https://github.com/{owner}/{repo}.git");
    let tmp = format!(".{repo}-clone");
    let _ = std::fs::remove_dir_all(&tmp);

    terminal_quiet(
        "git",
        &[
            "clone",
            "--quiet",
            "--depth=1",
            "--filter=blob:none",
            "--no-checkout",
            &url,
            &tmp,
        ],
    )?;

    let sources: Vec<&str> = mapping.iter().map(|(s, _)| *s).collect();
    let mut args = vec!["-C", &tmp, "sparse-checkout", "set", "--no-cone"];
    args.extend_from_slice(&sources);
    terminal_quiet("git", &args)?;

    terminal_quiet("git", &["-C", &tmp, "checkout", "--quiet"])?;

    std::fs::create_dir_all(dest)?;
    for (src_rel, dst_rel) in mapping {
        let src = format!("{tmp}/{src_rel}");
        let target = std::path::Path::new(dest).join(dst_rel);
        if std::fs::rename(&src, &target).is_err() {
            move_path(&src, &target)?;
        }
    }

    let _ = std::fs::remove_dir_all(&tmp);
    Ok(())
}
/// Moves a file or directory from `src` to `dst`, overwriting existing
/// files. Falls back to recursive copy for cross-device moves (EXDEV).
fn move_path(src: &str, dst: &std::path::Path) -> Result<()> {
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)?;
    }

    #[cfg(windows)]
    let _ = std::fs::remove_file(dst);

    if std::fs::rename(src, dst).is_ok() {
        return Ok(());
    }

    let mut stack = vec![(std::path::PathBuf::from(src), dst.to_path_buf())];
    while let Some((s, d)) = stack.pop() {
        std::fs::create_dir_all(&d)?;
        for entry in std::fs::read_dir(&s)? {
            let entry = entry?;
            let target = d.join(entry.file_name());
            if entry.path().is_dir() {
                stack.push((entry.path(), target));
            } else {
                std::fs::copy(entry.path(), &target)?;
            }
        }
    }
    Ok(())
}
pub fn is_installed(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}
pub fn open_browser(url: &str) -> Result<()> {
    let result = if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(url).status()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(url).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", url]).status()
    } else {
        return Err("Unsupported OS".into());
    };

    result?;
    Ok(())
}
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_installed_returns_true_for_existing_program() {
        assert!(is_installed("cargo"));
    }

    #[test]
    fn is_installed_returns_false_for_non_existing_program() {
        assert!(!is_installed("random_non_existing_program_xyz_123"));
    }

    #[test]
    fn is_installed_returns_false_for_empty_string() {
        assert!(!is_installed(""));
    }

    #[test]
    fn capitalize_returns_same_string_when_already_capitalized() {
        assert_eq!(capitalize("Back"), "Back");
        assert_eq!(capitalize("Hello"), "Hello");
    }

    #[test]
    fn capitalize_uppercases_first_letter() {
        assert_eq!(capitalize("back"), "Back");
        assert_eq!(capitalize("hello world"), "Hello world");
    }

    #[test]
    fn capitalize_returns_empty_for_empty_string() {
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn capitalize_handles_single_letter() {
        assert_eq!(capitalize("a"), "A");
        assert_eq!(capitalize("Z"), "Z");
    }

    #[test]
    fn capitalize_handles_unicode() {
        assert_eq!(capitalize("żółw"), "Żółw");
        assert_eq!(capitalize("ćma"), "Ćma");
    }

    #[test]
    fn capitalize_keeps_leading_digit_unchanged() {
        assert_eq!(capitalize("123abc"), "123abc");
    }

    #[test]
    fn capitalize_keeps_leading_space_unchanged() {
        assert_eq!(capitalize(" hello"), " hello");
    }

    #[test]
    fn capitalize_expands_sharp_s_to_two_chars() {
        assert_eq!(capitalize("ß"), "SS");
    }
}
