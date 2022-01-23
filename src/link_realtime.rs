use std::env;

use camino::Utf8Path;
use camino::Utf8PathBuf;
use mslnk::{ShellLink, ShowCommand};
use stable_eyre::{eyre::bail, Result};

pub(crate) fn make_link(target: &Utf8Path, location: &Utf8Path) -> Result<()> {
    let system_root = env::var("SYSTEMROOT")?;
    let mut link = ShellLink::new(format!(
        r"{system_root}\System32\WindowsPowerShell\v1.0\powershell.exe"
    ))?;
    let self_path = Utf8PathBuf::try_from(env::current_exe()?)?;
    link.set_arguments(Some(format!(
        "-Command Start-Process {self_path} -ArgumentList \"--background\",\"launch\",\"{target}\" -WindowStyle Hidden -Verb RunAs")));
    match target.parent() {
        Some(parent) => link.set_working_dir(Some(parent.to_string())),
        None => bail!("Unable to determine parent directory of shortcut target"),
    }
    link.set_icon_location(Some(target.to_string()));
    link.header_mut()
        .set_show_command(ShowCommand::ShowMinNoActive);

    link.create_lnk(location.with_extension("lnk"))?;
    Ok(())
}
