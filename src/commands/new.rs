use cliclack::select;
use crate::{commands, utilities::{git_folder, terminal}};

pub fn run(project_name: &str) {

    let choice = select("What type of project?")
        .item("application", "Application", "desktop or CLI app")
        .item("website", "Website", "web project")
        .item("module", "Module", "library module")
        .item("back", "Back", "back to main menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    match choice.as_ref() {
        "Application" => git_folder("Secure-Your-Soul", "SoulsCLI", "templates/application", project_name),
        "Website" => git_folder("Secure-Your-Soul", "SoulsCLI", "templates/website", project_name),
        "Module" => git_folder("Secure-Your-Soul", "SoulsCLI", "templates/module", project_name),
        "Back" => commands::menu::run(),
        _ => unreachable!(),
    }
    terminal("git", &["-C", project_name, "init"]);
    terminal("git", &["-C", project_name, "add", "."]);
    terminal("git", &["-C", project_name, "commit", "-m", "SoulsCLI: Initial Commit"]);
    terminal("git", &["-C", project_name, "push"]);
}