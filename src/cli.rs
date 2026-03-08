use crate::errorbox;
use crate::launch::launch;
use crate::link::make_link;
use crate::pathext::Utf8PathExt;
use camino::Utf8Path;
use clap::{ArgMatches, Command, arg};
use stable_eyre::Result;
use stable_eyre::eyre::{bail, eyre};

fn run_subcommand(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("launch", sub_matches)) => {
            let program = Utf8Path::new(
                sub_matches
                    .get_one::<&str>("PROGRAM")
                    .ok_or_else(|| eyre!("PROGRAM is required"))?,
            )
            .absolutize()?;

            if !program.exists() {
                bail!("Program must exist")
            }

            launch(&program, matches.contains_id("background"))?;
        }
        Some(("link", sub_matches)) => {
            let target = Utf8Path::new(
                sub_matches
                    .get_one::<&str>("TARGET")
                    .ok_or_else(|| eyre!("TARGET is required"))?,
            )
            .absolutize()?;
            let location = Utf8Path::new(
                sub_matches
                    .get_one::<&str>("LOCATION")
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

pub(crate) fn run() -> Result<(), stable_eyre::Report> {
    let matches = Command::new("ampsim_launcher")
        .about("A tool for launching programs with optimized performance")
        .arg(arg!(-b --background "Activate background mode (errors as message boxes, hidden console window)"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("launch")
                .about("Launch a process with high performance settings")
                .arg(arg!(<PROGRAM> "The program to launch"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("link")
                .about("Create a shortcut for a given program")
                .arg(arg!(<TARGET> "Location of the target program"))
                .arg(arg!(<LOCATION> "Location of the shortcut"))
                .arg_required_else_help(true),
        )
        .get_matches();
    let result = run_subcommand(&matches);
    if matches.contains_id("background") {
        match result {
            Ok(()) => (),
            Err(ref e) => errorbox::show(format!("{e:#}").as_str()),
        }
    }
    result
}
