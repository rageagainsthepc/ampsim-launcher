use std::path::Path;

use mslnk::ShellLink;
use stable_eyre::Result;

pub(crate) fn make_link(target: &Path, location: &Path) -> Result<()> {
    let mut link = ShellLink::new(std::env::current_exe()?)?;
    link.set_arguments(Some(format!("-e launch \"{}\"", target.to_string_lossy())));
    link.set_icon_location(Some(target.to_str().unwrap().to_string()));

    link.create_lnk(location.with_extension("lnk"))?;
    Ok(())
}
