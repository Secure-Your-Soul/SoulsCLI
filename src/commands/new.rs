use cliclack::{select, outro, spinner, log};
use crate::{commands, utilities::{git_folder, terminal}};

pub fn run(project_name: &str) {

    let spinner = spinner();
    let choice = select("Choose enviroment")
        .item("application", "Application", "Desktop Web Mobile")
        .item("website", "Website", "Web")
        .item("module", "Module", "Bin/Rust")
        .item("back", "Back", "Back to main menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));
    
    spinner.start(format!("Creating {project_name}"));

    match choice.as_ref() {
        "application" => git_folder("Secure-Your-Soul", "SoulsCLI", "templates/application", project_name),
        "website" => git_folder("Secure-Your-Soul", "SoulsCLI", "templates/website", project_name),
        "module" => git_folder("Secure-Your-Soul", "SoulsCLI", "templates/module", project_name),
        "back" => { spinner.stop(""); commands::menu::run(); return; },
        _ => unreachable!(),
    }

    terminal("git", &["-C", project_name, "init"]);
    terminal("git", &["-C", project_name, "add", "."]);
    terminal("git", &["-C", project_name, "commit", "-m", "chore(SoulsCLI): initial commit"]);
    
    spinner.stop("");
    log::success(format!("Project created in {project_name}")).unwrap();
    outro("").unwrap();
}