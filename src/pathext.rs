use std::{fs, io, path::Path, result::Result};

pub(crate) trait PathExt {
    /// Like `std::path::Path::is_dir()` but returns a result object in order to
    /// be able to differentiate between an actual result and io errors.
    fn is_dir_ext(&self) -> Result<bool, io::Error>;
    /// Like `std::path::Path::is_file()` but returns a result object in order to
    /// be able to differentiate between an actual result and io errors.
    fn is_file_ext(&self) -> Result<bool, io::Error>;
}

impl PathExt for Path {
    fn is_dir_ext(&self) -> Result<bool, io::Error> {
        fs::metadata(self).map(|m| m.is_dir())
    }

    fn is_file_ext(&self) -> Result<bool, io::Error> {
        fs::metadata(self).map(|m| m.is_file())
    }
}
