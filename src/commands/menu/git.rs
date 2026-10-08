use crate::utilities::{Result, capitalize, terminal};
use cliclack::{clear_screen, confirm, input, log, select};
pub fn git_menu() -> Result<()> {
    loop {
        clear_screen()?;
        let category = select("")
            .item("work", "Work", "add, amend, commit, restore")
            .item("inspect", "Inspect", "status, diff, log, show")
            .item(
                "history",
                "History",
                "branch, merge, rebase, reset, switch, tag",
            )
            .item("remote", "Remote", "fetch, pull, push")
            .item("back", "Back", "back to main menu")
            .interact()?;

        match category {
            "work" => git_work()?,
            "inspect" => git_inspect()?,
            "history" => git_history()?,
            "remote" => git_remote()?,
            "back" => return Ok(()),
            _ => unreachable!(),
        }
    }
}
pub fn git_work() -> Result<()> {
    loop {
        let choice = select("")
            .item("add", "Add", "stage all changes")
            .item("amend", "Amend", "change the last commit message")
            .item("commit", "Commit", "commit with default message")
            .item("restore", "Restore", "restore working tree")
            .item("back", "Back", "back to Git menu")
            .interact()?;

        match choice {
            "add" => terminal("git", &["add", "-A"])?,
            "amend" => terminal("git", &["commit", "--amend"])?,
            "commit" => {
                let has_changes = std::process::Command::new("git")
                    .args(["diff", "--cached", "--quiet"])
                    .status()?
                    .code()
                    == Some(1); // 1 = changes, 0 = none

                if !has_changes {
                    log::warning("nothing staged — run `git add` first")?;
                    continue;
                }
                let title: String = input("Commit title")
                    .default_input("chore: SoulsCLI default commit")
                    .placeholder("chore: write proper commit message")
                    .interact()?;

                let body: String = input("Commit body (optional)")
                    .placeholder("Optional description (e.g. - fix bug - add feature)")
                    .interact()?;

                let msg = if body.is_empty() {
                    title
                } else {
                    format!("{title}\n\n{body}")
                };
                terminal("git", &["commit", "-m", &msg])?;
            }
            "restore" => {
                if confirm("Discard all working tree changes?").interact()? {
                    terminal("git", &["restore", "."])?;
                }
            }
            "back" => return Ok(()),
            _ => unreachable!(),
        }
    }
}
pub fn git_inspect() -> Result<()> {
    loop {
        let choice = select("")
            .item("status", "Status", "working tree status")
            .item("diff", "Diff", "show unstaged changes")
            .item("log", "Log", "show last 20 commits")
            .item("show", "Show", "show current commit")
            .item("back", "Back", "back to Git menu")
            .interact()?;

        match choice {
            "status" => terminal("git", &["status"])?,
            "diff" => terminal("git", &["diff"])?,
            "log" => terminal("git", &["log", "--oneline", "-20"])?,
            "show" => terminal("git", &["show"])?,
            "back" => return Ok(()),
            _ => unreachable!(),
        }
    }
}
pub fn git_history() -> Result<()> {
    loop {
        let choice = select("")
            .item("branch", "Branch", "list branches")
            .item("tag", "Tag", "list tags")
            .item("reset", "Reset", "reset index")
            .item("switch", "Switch", "switch branch")
            .item("merge", "Merge", "merge branch")
            .item("rebase", "Rebase", "rebase onto branch")
            .item("back", "Back", "back to Git menu")
            .interact()?;

        match choice {
            "branch" => terminal("git", &["branch"])?,
            "tag" => terminal("git", &["tag"])?,
            "reset" => terminal("git", &["reset"])?,
            "switch" => {
                let branch: String = input("Branch?").default_input("stable").interact()?;

                terminal("git", &["switch", &branch])?;
            }
            "merge" => {
                let branch: String = input("Branch to merge?")
                    .default_input("stable")
                    .interact()?;

                terminal("git", &["merge", &branch])?;
            }
            "rebase" => {
                let mode = select("Rebase range")
                    .item("root", "From root", "all commits from the beginning")
                    .item("recent", "Recent", "last N commits")
                    .item("onto", "Onto branch", "move commits onto another branch")
                    .item("back", "Back", "back to History menu")
                    .interact()?;

                match mode {
                    "root" => terminal("git", &["rebase", "-i", "--autostash", "--root"])?,
                    "recent" => {
                        let n: String = input("How many commits back?")
                            .default_input("5")
                            .interact()?;
                        terminal(
                            "git",
                            &["rebase", "-i", "--autostash", &format!("HEAD~{n}")],
                        )?;
                    }
                    "onto" => {
                        let branch: String = input("Branch?").default_input("stable").interact()?;
                        terminal("git", &["rebase", "-i", "--autostash", &branch])?;
                    }
                    "back" => (),
                    _ => unreachable!(),
                }
            }
            "back" => return Ok(()),
            _ => unreachable!(),
        }
    }
}
pub fn git_remote() -> Result<()> {
    loop {
        let choice = select("")
            .item("fetch", "Fetch", "download objects and refs")
            .item("pull", "Pull", "pull from remote")
            .item("push", "Push", "push to remote")
            .item("back", "Back", "back to Git menu")
            .interact()?;

        if choice != "back" {
            log::step(format!("Git: {}", capitalize(choice)))?;
        }

        match choice {
            "fetch" => terminal("git", &["fetch"])?,
            "pull" => terminal("git", &["pull"])?,
            "push" => terminal("git", &["push", "-u", "origin", "HEAD"])?,
            "back" => return Ok(()),
            _ => unreachable!(),
        }
        log::success(format!("Git: {}", capitalize(choice)))?;
    }
}
