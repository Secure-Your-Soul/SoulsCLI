use crate::{
    commands,
    utilities::{Result, git_paths, terminal_quiet},
};
use cliclack::{clear_screen, input, log, select, spinner};
pub fn run(project_name: &str) -> Result<()> {
    clear_screen()?;
    let owned;
    let project_name = if project_name.is_empty() {
        owned = input("Project name")
            .default_input("souls-project")
            .placeholder("souls-project")
            .autocomplete(vec!["souls-project".to_string()])
            .interact::<String>()?;
        &owned
    } else {
        project_name
    };

    let choice = select("Choose template")
        .item("application", "Application", "Desktop Web Mobile")
        .item("website", "Website", "Web")
        .item("module", "Module", "Bin/Rust")
        .item("back", "Back", "Back to main menu")
        .interact()?;

    let template = match choice {
        "application" => "application",
        "website" => "website",
        "module" => "module",
        "back" => {
            commands::menu::run()?;
            return Ok(());
        }
        _ => unreachable!(),
    };

    let sp = spinner();
    let sp1 = spinner();
    let sp2 = spinner();
    let sp3 = spinner();
    sp.start("Downloading template");
    let template_path = format!("templates/{template}");
    git_paths(
        "Secure-Your-Soul",
        "SoulsCLI",
        &[&template_path],
        project_name,
    )?;
    sp.stop("Template downloaded");
    sp1.start("Initialazing repository");
    terminal_quiet("git", &["-C", project_name, "init", "--quiet"])?;
    sp1.stop("Initiliazed repository");
    sp2.start("Add files to stage");
    terminal_quiet("git", &["-C", project_name, "add", "."])?;
    sp2.stop("Added files to stage");
    sp3.start("Commiting project");
    terminal_quiet(
        "git",
        &[
            "-C",
            project_name,
            "commit",
            "--quiet",
            "-m",
            "chore(SoulsCLI): initial commit",
        ],
    )?;
    sp3.stop("Project commited");
    log::success("Project created")?;
    log::remark(format!("Go into project: cd {project_name}"))?;
    Ok(())
}
