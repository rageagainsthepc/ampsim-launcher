mod launch;
mod link;

use std::path::Path;

use crate::launch::launch;
use crate::link::make_link;
use clap::{arg, App, AppSettings};
use path_absolutize::Absolutize;
use stable_eyre::eyre::{bail, eyre};
use stable_eyre::Result;
use windows::Win32::System::Console::FreeConsole;

fn hide_console_window() {
    unsafe {
        FreeConsole();
    }
}

fn main() -> Result<()> {
    stable_eyre::install()?;

    let matches = App::new("ampsim_starter")
        .about("A tool for launching programs with optimized performance")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("launch")
                .about("Launch a process with high performance settings")
                .arg(arg!(<PROGRAM> "The program to launch"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("link")
                .about("Creates a shortcut for a given program")
                .arg(arg!(<TARGET> "Location of the target program"))
                .arg(arg!(<LOCATION> "Location of the shortcut"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("launch", sub_matches)) => {
            let program = Path::new(
                sub_matches
                    .value_of("PROGRAM")
                    .ok_or_else(|| eyre!("PRORGAM is required"))?,
            )
            .absolutize()?;

            if !program.exists() {
                bail!("Program must exist")
            }

            hide_console_window();
            launch(&program)?;
        }
        Some(("link", sub_matches)) => {
            let target = Path::new(
                sub_matches
                    .value_of("TARGET")
                    .ok_or_else(|| eyre!("TARGET is required"))?,
            )
            .absolutize()?;
            let location = Path::new(
                sub_matches
                    .value_of("LOCATION")
                    .ok_or_else(|| eyre!("LOCATION is required"))?,
            )
            .absolutize()?;

            if !target.exists() {
                bail!("Target must exist")
            }

            make_link(&target, &location)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
