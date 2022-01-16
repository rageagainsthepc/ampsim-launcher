use std::{env, io};

use camino::{Utf8Path, Utf8PathBuf};
use stable_eyre::Result;

use crate::{errorbox, link::make_link, pathext::Utf8PathExt};

fn get_target_path() -> Result<Utf8PathBuf> {
    let target_path = loop {
        let mut input_buffer = String::new();
        std::io::stdin().read_line(&mut input_buffer)?;
        let mut line = input_buffer.lines().next().unwrap();
        line = quoted_string::strip_dquotes(line).unwrap_or(line);

        if line.is_empty() {
            println!("Input required. Enter the path of a target executable:");
        } else {
            match Utf8Path::new(line).is_file_ext() {
                Ok(is_file) => {
                    if is_file {
                        break Utf8PathBuf::from(line);
                    }
                    println!("Target does not exist or is not a file.");
                }
                Err(e) => println!("{e}"),
            }
        }
        println!("Enter the path of a target executable:");
    };
    Ok(target_path)
}

fn get_shortcut_path(target_path: &Utf8Path) -> Result<Utf8PathBuf> {
    let default_shortcut_path = Utf8Path::new(&env::var("USERPROFILE")?)
        .join("Desktop")
        .join(
            Utf8Path::new(target_path)
                .with_extension("lnk")
                .file_name()
                .unwrap(),
        );
    println!(
        "Enter the path where the shortcut will be created (default: {default_shortcut_path}):"
    );

    let shortcut_path = loop {
        let mut input_buffer = String::new();
        std::io::stdin().read_line(&mut input_buffer)?;
        let mut line = input_buffer.lines().next().unwrap();
        line = quoted_string::strip_dquotes(line).unwrap_or(line);

        if line.is_empty() {
            break None;
        } else if let Some(p) = Utf8Path::new(line).absolutize()?.parent() {
            match p.is_dir_ext() {
                Ok(is_dir) => {
                    if is_dir {
                        break Some(Utf8PathBuf::from(line));
                    }
                    println!("Parent directory is not a directory.");
                }
                Err(e) => println!("{e}"),
            }
        }
        println!("Enter a valid shortcut path:");
    };

    Ok(shortcut_path.unwrap_or(default_shortcut_path))
}

fn create_shortcut() -> Result<()> {
    println!("Shortcut Creation Mode");
    println!("Enter the path of a target executable:");

    let target_path = get_target_path()?;
    let shortcut_path = get_shortcut_path(&target_path)?;

    make_link(target_path.as_path(), shortcut_path.as_path())?;

    println!("Shortcut created sucessfully. Press Enter to terminate...");
    let mut finish = String::new();
    io::stdin().read_line(&mut finish)?;

    Ok(())
}

pub(crate) fn run() -> Result<()> {
    let result = create_shortcut();

    if let Err(ref e) = result {
        errorbox::show(format!("{e:#}").as_str())
    }

    result
}
