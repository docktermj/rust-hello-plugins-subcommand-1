use clap::{Arg, Command};

pub fn cli() -> Command {
    subcommand("subcommand1")
        .about("Create a new cargo package in an existing directory")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .action(ArgAction::Set)
                .default_value("."),
        )
        .arg_new_opts()
        .arg_registry("Registry to use")
        .arg_silent_suggestion()
        .after_help(color_print::cstr!(
            "Run `<bright-cyan,bold>cargo help init</>` for more detailed information.\n"
        ))
}
