use camino::Utf8Path;
use mslnk::{ShellLink, ShowCommand};
use stable_eyre::{eyre::bail, Result};

pub(crate) fn make_link(target: &Utf8Path, location: &Utf8Path) -> Result<()> {
    let mut link = ShellLink::new(std::env::current_exe()?)?;
    link.set_arguments(Some(format!("--background launch \"{target}\"")));
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
