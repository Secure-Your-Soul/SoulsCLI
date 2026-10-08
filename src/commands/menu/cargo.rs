use crate::utilities::{Result, terminal};
use cliclack::select;
pub fn cargo_menu() -> Result<()> {
    loop {
        let choice = select("")
            .item("check", "Check", "check without building")
            .item("build", "Build", "compile the project")
            .item("test", "Test", "run tests")
            .item("fmt", "Format", "format code with rustfmt")
            .item("lint", "Lint", "run clippy lints")
            .item("doc", "Doc", "generate documentation")
            .item("clean", "Clean", "remove target directory")
            .item("update", "Update", "update dependencies")
            .item("back", "Back", "back to main menu")
            .interact()?;

        match choice {
            "check" => terminal("cargo", &["check", "--workspace"])?,
            "build" => terminal("cargo", &["build", "--workspace"])?,
            "test" => terminal("cargo", &["test", "--workspace"])?,
            "fmt" => terminal("cargo", &["fmt", "--all"])?,
            "lint" => terminal("cargo", &["clippy", "--workspace", "--", "-D", "warnings"])?,
            "doc" => terminal("cargo", &["doc", "--workspace", "--open"])?,
            "clean" => terminal("cargo", &["clean", "--workspace"])?,
            "update" => terminal("cargo", &["update", "--workspace"])?,
            "back" => return Ok(()),
            _ => unreachable!(),
        }
    }
}
