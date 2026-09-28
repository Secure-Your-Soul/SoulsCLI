use cliclack::{
    select, input, intro, outro, spinner,
    //log, multiselect, password, confirm, progress_bar, multi_progress
};
use crate::{commands::{new, menu}, utilities::{terminal, is_installed, open_browser, capitalize}};
use std::{ sync::atomic::{AtomicBool, Ordering}};
static WAS_INTRO: AtomicBool = AtomicBool::new(false);
pub fn run() {
    if !WAS_INTRO.swap(true, Ordering::SeqCst) { intro("Souls").unwrap(); }
    let mut menu = select("");
    let is_git = is_installed("git");
    if is_git { menu = menu.item("new", "New Project", "Create a new project"); }
    if is_installed("cargo") { menu = menu.item("cargo", "Cargo", "Manage cargo project"); } else { menu = menu.item("nocargo", "Cargo", "Click to install"); }
    if is_git { menu = menu.item("git", "Git", "Manage git operations"); } else { menu = menu.item("nogit", "Git", "Click to install"); }

    let choice = menu
//      .item("update", "Update", "")
        .item("exit", "Exit", "Quit the program")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));
    
    match choice.as_ref() {
        "new" => {
            let name: String = input("Project name")
                .default_input("souls-project")
                .placeholder("souls-project")
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));
            new::run(&name);
        },
        "cargo" => cargo_menu(),
        "nocargo" => open_browser("https://rust-lang.org/tools/install/"),
        "git" => git_menu(),
        "nogit" => open_browser("https://git-scm.com/install/"),
        "update" => {},
        "exit" => { outro("").unwrap(); std::process::exit(0); },
        _ => unreachable!(),
    }
}

fn cargo_menu() {
    let spinner = spinner();
    let cargo_choice = select("")
            .item("check", "Check", "check without building")
            .item("build", "Build", "compile the project")
            .item("test", "Test", "run tests")
            .item("fmt", "Format", "format code with rustfmt")
            .item("lint", "Lint", "run clippy lints")
            .item("doc", "Doc", "generate documentation")
            .item("clean", "Clean", "remove target directory")
            .item("update", "Update", "update dependencies")
            .item("back", "Back", "back to main menu")
            .interact()
            .unwrap_or_else(|_| std::process::exit(0));

    spinner.start(format!("Cargo: {}", capitalize(cargo_choice)));

    match cargo_choice.as_ref() {
        "check" => terminal("cargo", &["check", "--workspace"]),
        "build" => terminal("cargo", &["build", "--workspace"]),
        "test" => terminal("cargo", &["test", "--workspace"]),
        "fmt" => terminal("cargo", &["fmt", "--all"]),
        "lint" => terminal("cargo", &["clippy", "--workspace", "--", "-D", "warnings"]),
        "doc" => terminal("cargo", &["doc", "--workspace", "--open"]),
        "clean" => terminal("cargo", &["clean", "--workspace"]),
        "update" => terminal("cargo", &["update", "--workspace"]),
        "back" => { spinner.stop("Back"); menu::run(); return; }
        _ => unreachable!(),
    }
    spinner.stop(format!("Cargo: {}", capitalize(cargo_choice)));
    cargo_menu();
}

fn git_menu() {
    let category = select("")
        .item("work", "Work", "add, amend, commit, restore")
        .item("inspect", "Inspect", "status, diff, log, show")
        .item("history", "History", "branch, merge, rebase, reset, switch, tag")
        .item("remote", "Remote", "fetch, pull, push")
        .item("back", "Back", "back to main menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    match category.as_ref() {
        "work" => git_work(),
        "inspect" => git_inspect(),
        "history" => git_history(),
        "remote" => git_remote(),
        "back" => run(),
        _ => unreachable!(),
    }
}

fn git_work() {
    let spinner = spinner();
    let choice = select("")
        .item("add", "Add", "stage all changes")
        .item("amend", "Amend", "change the last commit message")
        .item("commit", "Commit", "commit with default message")
        .item("restore", "Restore", "restore working tree")
        .item("back", "Back", "back to Git menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    
    if choice != "commit" { spinner.start(format!("Git: {}", capitalize(choice))); };
    
    match choice.as_ref() {
        "add" => terminal("git", &["add", "."]),
        "amend" => terminal("git", &["commit", "--amend"]),
        "commit" => {
            let title: String = input("Commit title")
                .default_input("chore: SoulsCLI default commit")
                .placeholder("chore: write proper commit message")
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));

            let body: String = input("Commit body (optional)")
                .placeholder("Optional description (e.g. - fix bug - add feature)")
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));

            let msg = if body.is_empty() { title } else { format!("{title}\n\n{body}") };
            spinner.start(format!("Git: {}", capitalize(choice)));
            terminal("git", &["commit", "-m", &msg]);
        }
        "restore" => terminal("git", &["restore", "."]),
        "back" => { spinner.stop(format!("Git: {}", capitalize(choice))); git_menu(); return; },
        _ => unreachable!(),
    }
    spinner.stop(format!("Git: {}", capitalize(choice)));
    git_work();
}

fn git_inspect() {
    let spinner = spinner();
    let choice = select("")
        .item("status", "Status", "working tree status")
        .item("diff", "Diff", "show unstaged changes")
        .item("log", "Log", "show last 20 commits")
        .item("show", "Show", "show current commit")
        .item("back", "Back", "back to Git menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    spinner.start(format!("Git: {}", capitalize(choice)));

    match choice.as_ref() {
        "status" => terminal("git", &["status"]),
        "diff" => terminal("git", &["diff"]),
        "log" => terminal("git", &["log", "--oneline", "-20"]),
        "show" => terminal("git", &["show"]),
        "back" => { spinner.stop(format!("Git: {}", capitalize(choice))); git_menu(); return; },
        _ => unreachable!(),
    }
    spinner.stop(format!("Git: {}", capitalize(choice)));
    git_inspect();
}

fn git_history() {
    let spinner = spinner();
    let choice = select("")
        .item("branch", "Branch", "list branches")
        .item("tag", "Tag", "list tags")
        .item("reset", "Reset", "reset index")
        .item("switch", "Switch", "switch branch")
        .item("merge", "Merge", "merge branch")
        .item("rebase", "Rebase", "rebase onto branch")
        .item("back", "Back", "back to Git menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    if !["switch", "merge", "rebase"].contains(&choice.as_ref()) { spinner.start(format!("Git: {}", capitalize(choice))); }

    match choice.as_ref() {
        "branch" => { terminal("git", &["branch"]); git_history(); }
        "tag" => { terminal("git", &["tag"]); git_history(); }
        "reset" => { terminal("git", &["reset"]); git_history(); }
        "switch" => {
            let branch: String = input("Branch?")
                .default_input("stable")
                .interact()
                .unwrap();

            spinner.start(format!("Git: {}", capitalize(choice)));
            terminal("git", &["switch", &branch]);
        }
        "merge" => {
            let branch: String = input("Branch to merge?")
                .default_input("stable")
                .interact()
                .unwrap();

            spinner.start(format!("Git: {}", capitalize(choice)));
            terminal("git", &["merge", &branch]);
        }
        "rebase" => {
            let branch: String = input("Branch to rebase into?")
                .default_input("stable")
                .interact()
                .unwrap();

            spinner.start(format!("Git: {}", capitalize(choice)));
            terminal("git", &["rebase", "-i", "--root", "--autostash", &branch]);
        }
        "back" => { spinner.stop(format!("Git: {}", capitalize(choice))); git_menu(); return; },
        _ => unreachable!(),
    }
    spinner.stop(format!("Git: {}", capitalize(choice)));
    git_history();
}

fn git_remote() {
    let spinner = spinner();
    let choice = select("")
        .item("fetch", "Fetch", "download objects and refs")
        .item("pull", "Pull", "pull from remote")
        .item("push", "Push", "push to remote")
        .item("back", "Back", "back to Git menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    spinner.start(format!("Git: {}", capitalize(choice)));

    match choice.as_ref() {
        "fetch" => terminal("git", &["fetch"]),
        "pull" => terminal("git", &["pull"]),
        "push" => terminal("git", &["push"]),
        "back" => { spinner.stop(format!("Git: {}", capitalize(choice))); git_menu(); return; },
        _ => unreachable!(),
    }
    spinner.stop(format!("Git: {}", capitalize(choice)));
    git_remote();
}