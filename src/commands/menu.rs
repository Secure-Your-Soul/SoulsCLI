#[allow(unused)]
use cliclack::{
    select, multiselect, input, password, confirm,
    intro, outro, log,
    spinner, progress_bar, multi_progress,
};
use crate::{commands, utilities::terminal};
pub fn run() {
    intro("SoulsCLI").unwrap();

    let choice = select("What do you want to do?")
        .item("new", "New Project", "create a new project")
        .item("cargo", "Cargo", "manage cargo crates")
        .item("git", "Git", "git operations")
        .item("exit", "Exit", "quit the program")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    outro(format!("Selected: {choice}")).unwrap();

    match choice.as_ref() {
        "new" => {
            let name: String = input("Project name?")
                .default_input("souls-project")
                .placeholder("souls-project")
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));
            commands::new::run(&name);
        },
        "cargo" => {
            let cargo_choice = select("Cargo:")
            //          Id       Label           Hint
                .item("check", "Check", "check without building")
                .item("build", "Build", "compile the project")
                .item("test", "Test", "run tests")
                .item("back", "Back", "back to main menu")
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));

            match cargo_choice.as_ref() {
                "check" => terminal("cargo", &["check"]),
                "build" => terminal("cargo", &["build"]),
                "test" => terminal("cargo", &["test"]),
                "back" => commands::menu::run(),
                _ => unreachable!(),
            }
        }
        "git" => git_menu(),
        "exit" => std::process::exit(0),
        _ => unreachable!(),
    }
}

fn git_menu() {
    let category = select("Git category:")
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
        "back" => run(),  // ← wróć do głównego menu
        _ => unreachable!(),
    }
}

fn git_work() {
    let choice = select("Git work:")
        .item("add", "Add", "stage all changes")
        .item("amend", "Amend", "change the last commit message")
        .item("commit", "Commit", "commit with default message")
        .item("restore", "Restore", "restore working tree")
        .item("back", "Back", "back to Git menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    match choice.as_ref() {
        "add" => { terminal("git", &["add", "."]); git_work(); }
        "amend" => { terminal("git", &["commit", "--amend"]); git_work(); }
        "commit" => {
            let title: String = input("Commit title?")
                .default_input("chore: SoulsCLI default commit")
                .placeholder("chore: write proper commit message")
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));

            let body: String = input("Commit body? (optional)")
                .placeholder("Optional description (e.g. - fix bug - add feature)")
                .interact()
                .unwrap_or_else(|_| std::process::exit(0));

            let msg = if body.is_empty() { title } else { format!("{title}\n\n{body}") };
            terminal("git", &["commit", "-m", &msg]);
            git_work();
        }
        "restore" => { terminal("git", &["restore", "."]); git_work(); }
        "back" => git_menu(),  // ← wróć do Git menu
        _ => unreachable!(),
    }
}

fn git_inspect() {
    let choice = select("Git inspect:")
        .item("status", "Status", "working tree status")
        .item("diff", "Diff", "show unstaged changes")
        .item("log", "Log", "show last 20 commits")
        .item("show", "Show", "show current commit")
        .item("back", "Back", "back to Git menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    match choice.as_ref() {
        "status" => { terminal("git", &["status"]); git_inspect(); }
        "diff" => { terminal("git", &["diff"]); git_inspect(); }
        "log" => { terminal("git", &["log", "--oneline", "-20"]); git_inspect(); }
        "show" => { terminal("git", &["show"]); git_inspect(); }
        "back" => git_menu(),  // ← wróć do Git menu
        _ => unreachable!(),
    }
}

fn git_history() {
    let choice = select("Git history:")
        .item("branch", "Branch", "list branches")
        .item("tag", "Tag", "list tags")
        .item("reset", "Reset", "reset index")
        .item("switch", "Switch", "switch branch")
        .item("merge", "Merge", "merge branch")
        .item("rebase", "Rebase", "rebase onto branch")
        .item("back", "Back", "back to Git menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    match choice.as_ref() {
        "branch" => { terminal("git", &["branch"]); git_history(); }
        "tag" => { terminal("git", &["tag"]); git_history(); }
        "reset" => { terminal("git", &["reset"]); git_history(); }
        "switch" => {
            let branch: String = input("Branch?")
                .default_input("stable")
                .interact()
                .unwrap();
            terminal("git", &["switch", &branch]);
            git_history();
        }
        "merge" => {
            let branch: String = input("Branch to merge?")
                .default_input("stable")
                .interact()
                .unwrap();
            terminal("git", &["merge", &branch]);
            git_history();
        }
        "rebase" => {
            let branch: String = input("Branch to rebase into?")
                .default_input("stable")
                .interact()
                .unwrap();
            terminal("git", &["rebase", "-i", "--root", "--autostash", &branch]);
            git_history();
        }
        "back" => git_menu(),  // ← wróć do Git menu
        _ => unreachable!(),
    }
}

fn git_remote() {
    let choice = select("Git remote:")
        .item("fetch", "Fetch", "download objects and refs")
        .item("pull", "Pull", "pull from remote")
        .item("push", "Push", "push to remote")
        .item("back", "Back", "back to Git menu")
        .interact()
        .unwrap_or_else(|_| std::process::exit(0));

    match choice.as_ref() {
        "fetch" => { terminal("git", &["fetch"]); git_remote(); }
        "pull" => { terminal("git", &["pull"]); git_remote(); }
        "push" => { terminal("git", &["push"]); git_remote(); }
        "back" => git_menu(),  // ← wróć do Git menu
        _ => unreachable!(),
    }
}