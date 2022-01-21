use crate::errorbox;
cfg_if! {
    if #[cfg(feature = "realtime")] {
        use crate::launch_realtime::launch;
        use crate::link_realtime::make_link;
    } else {
        use crate::launch::launch;
        use crate::link::make_link;
    }
}
use crate::pathext::Utf8PathExt;
use camino::Utf8Path;
use cfg_if::cfg_if;
use clap::{arg, App, AppSettings, ArgMatches};
use stable_eyre::eyre::{bail, eyre};
use stable_eyre::Result;

fn run_subcommand(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("launch", sub_matches)) => {
            let program = Utf8Path::new(
                sub_matches
                    .value_of("PROGRAM")
                    .ok_or_else(|| eyre!("PRORGAM is required"))?,
            )
            .absolutize()?;

            if !program.exists() {
                bail!("Program must exist")
            }

            launch(&program, matches.is_present("background"))?;
        }
        Some(("link", sub_matches)) => {
            let target = Utf8Path::new(
                sub_matches
                    .value_of("TARGET")
                    .ok_or_else(|| eyre!("TARGET is required"))?,
            )
            .absolutize()?;
            let location = Utf8Path::new(
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

pub(crate) fn run() -> Result<(), stable_eyre::Report> {
    let matches = App::new("ampsim_starter")
        .about("A tool for launching programs with optimized performance")
        .arg(arg!(-b --background "Activate background mode (errors as message boxes, hidden console window)"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("launch")
                .about("Launch a process with high performance settings")
                .arg(arg!(<PROGRAM> "The program to launch"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .subcommand(
            App::new("link")
                .about("Create a shortcut for a given program")
                .arg(arg!(<TARGET> "Location of the target program"))
                .arg(arg!(<LOCATION> "Location of the shortcut"))
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();
    let result = run_subcommand(&matches);
    if matches.is_present("background") {
        match result {
            Ok(()) => (),
            Err(ref e) => errorbox::show(format!("{e:#}").as_str()),
        }
    }
    result
}
