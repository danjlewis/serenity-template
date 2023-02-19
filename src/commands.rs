use serenity::framework::StandardFramework;

mod general;
mod help;

pub const COMMAND_PREFIX: &str = "!";

pub fn framework() -> StandardFramework {
    StandardFramework::new()
        .configure(|cfg| cfg.prefix(COMMAND_PREFIX))
        .group(&general::GENERAL_GROUP)
        .help(&help::HELP)
}
