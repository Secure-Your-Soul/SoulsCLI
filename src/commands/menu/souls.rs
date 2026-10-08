use crate::{
    commands::new,
    utilities::{Result, git_update, terminal},
};
use cliclack::{log, multiselect, select, spinner};
pub fn souls_menu() -> Result<()> {
    loop {
        let choice = select("")
            .item("test", "Test", "Run all tests")
            .item("run", "Run", "Open")
            .item("new", "New Project", "Create a new project")
            .item("update", "Update", "Open")
            .item("back", "Back", "back to main menu")
            .interact()?;

        match choice {
            "test" => {
                terminal("cargo", &["fmt", "--all"])?;

                terminal(
                    "cargo",
                    &[
                        "clippy",
                        "--workspace",
                        "--all-targets",
                        "--all-features",
                        "--",
                        "-D",
                        "warnings",
                        "-D",
                        "clippy::pedantic",
                        "-D",
                        "clippy::nursery",
                        "-D",
                        "clippy::cargo",
                        "-W",
                        "clippy::unwrap_used",
                        "-W",
                        "clippy::expect_used",
                    ],
                )?;
                terminal(
                    "cargo",
                    &[
                        "test",
                        "--workspace",
                        "--all-features",
                        "--all-targets",
                        "--no-fail-fast",
                    ],
                )?;
            }
            "run" => souls_run()?,
            "new" => new::run("")?,
            "update" => souls_update()?,
            "back" => return Ok(()),
            _ => unreachable!(),
        }
    }
}
pub fn souls_run() -> Result<()> {
    let choice = select("")
        .item("", "Comming Soon", "Unavailable")
        .item("back", "Back", "back to main menu")
        .interact()?;

    match choice {
        "" => {
            log::warning("`run` is coming soon")?;
            Ok(())
        }
        "back" => Ok(()),
        _ => unreachable!(),
    }
}
pub fn souls_update() -> Result<()> {
    let mapping =
        match multiselect("Souls: which files? (space = select, enter = confirm, esc = back)")
            .item(
                ("templates/application/README.md", "README.md"),
                "README",
                "",
            )
            .item(
                ("templates/application/LICENSE.md", "LICENSE.md"),
                "LICENSE",
                "",
            )
            .item(
                (
                    "templates/application/CODE_OF_CONDUCT.md",
                    "CODE_OF_CONDUCT.md",
                ),
                "CODE OF CONDUCT",
                "",
            )
            .interact()
        {
            Ok(m) => m,
            Err(e) => {
                let msg = e.to_string();
                if msg.contains("Input required") || msg.contains("interrupted") {
                    return Ok(());
                }
                return Err(e.into());
            }
        };

    if mapping.is_empty() {
        return Ok(());
    }

    let sp = spinner();
    sp.start("Souls: fetching...");
    git_update("Secure-Your-Soul", "SoulsCLI", ".", &mapping)?;
    sp.stop("Souls: done");
    Ok(())
}
