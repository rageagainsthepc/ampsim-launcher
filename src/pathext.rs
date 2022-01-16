use std::{borrow::Cow, fs, io};

use camino::{Utf8Path, Utf8PathBuf};
use path_absolutize::Absolutize;

pub(crate) trait Utf8PathExt {
    /// Like `std::path::Path::is_dir()` but returns a result object in order to
    /// be able to differentiate between an actual result and io errors.
    fn is_dir_ext(&self) -> io::Result<bool>;
    /// Like `std::path::Path::is_file()` but returns a result object in order to
    /// be able to differentiate between an actual result and io errors.
    fn is_file_ext(&self) -> io::Result<bool>;
    /// Forward implementation for `absolutize()` which is only implemented
    /// for `std::path::Path`.
    fn absolutize(&self) -> io::Result<Cow<Utf8Path>>;
}

impl Utf8PathExt for Utf8Path {
    fn is_dir_ext(&self) -> io::Result<bool> {
        fs::metadata(self).map(|m| m.is_dir())
    }

    fn is_file_ext(&self) -> io::Result<bool> {
        fs::metadata(self).map(|m| m.is_file())
    }

    fn absolutize(&self) -> io::Result<Cow<Utf8Path>> {
        match self.as_std_path().absolutize()? {
            Cow::Borrowed(p) => Ok(Cow::Borrowed(Utf8Path::from_path(p).unwrap())),
            Cow::Owned(pb) => Ok(Cow::Owned(Utf8PathBuf::from_path_buf(pb).unwrap())),
        }
    }
}
