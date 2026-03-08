mod cli;
mod errorbox;
mod interactive;
mod launch;
mod link;
mod pathext;

use stable_eyre::Result;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

fn parent_name() -> String {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    let parent_id = system
        .process((std::process::id() as usize).into())
        .unwrap()
        .parent()
        .unwrap();
    let parent_name = system.process(parent_id).unwrap().name();
    parent_name.to_string_lossy().to_string()
}

fn main() -> Result<()> {
    stable_eyre::install()?;

    if std::env::args().len() < 2 && parent_name() == "explorer.exe" {
        interactive::run()
    } else {
        cli::run()
    }
}
