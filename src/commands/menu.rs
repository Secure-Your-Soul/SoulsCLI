pub mod cargo;
pub mod git;
pub mod souls;
use crate::{
    commands::menu::{cargo::cargo_menu, git::git_menu, souls::souls_menu},
    utilities::{Result, is_installed, open_browser},
};
use cliclack::{
    clear_screen,
    intro,
    outro,
    select,
    //input, log, multiselect, password, confirm, spinner, progress_bar, multi_progress
};
use std::sync::atomic::{AtomicBool, Ordering};
static WAS_INTRO: AtomicBool = AtomicBool::new(false);
pub fn run() -> Result<()> {
    loop {
        clear_screen()?;
        if !WAS_INTRO.swap(true, Ordering::SeqCst) {
            intro("Souls")?;
        }
        let mut menu = select("");
        let is_cargo = is_installed("cargo");
        let is_git = is_installed("git");
        if is_cargo && is_git {
            menu = menu.item("souls", "Souls", "Open");
        }
        if is_cargo {
            menu = menu.item("cargo", "Cargo", "Open");
        } else {
            menu = menu.item("nocargo", "Cargo", "Click to install");
        }
        if is_git {
            menu = menu.item("git", "Git", "Open");
        } else {
            menu = menu.item("nogit", "Git", "Click to install");
        }

        let choice = menu.item("exit", "Exit", "Quit the program").interact()?;

        match choice {
            "souls" => souls_menu()?,
            "cargo" => cargo_menu()?,
            "nocargo" => open_browser("https://rust-lang.org/tools/install/")?,
            "git" => git_menu()?,
            "nogit" => open_browser("https://git-scm.com/install/")?,
            "exit" => {
                outro("")?;
                return Ok(());
            }
            _ => unreachable!(),
        }
    }
}
