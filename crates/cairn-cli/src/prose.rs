//! Offline advisory authoring. No grammar finding changes lifecycle authority.
use std::error::Error;

mod analysis;
mod check;
mod model;
mod profiles;
mod render;
mod select;
mod verify;

pub(crate) fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some((command, options)) = args.split_first() else {
        return Err("prose requires check, verify, or profiles export".into());
    };
    match command.as_str() {
        "check" => check::run(options),
        "verify" => verify::run(options),
        "profiles" if options.first().is_some_and(|option| option == "export") => {
            profiles::export(&options[1..])
        }
        _ => Err("unknown prose command; use check, verify, or profiles export".into()),
    }
}
