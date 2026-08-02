use std::env;
use std::io;
use std::path::PathBuf;

use directories::ProjectDirs;

const DATA_DIR_ENV: &str = "FENRIR_DATA_DIR";

pub fn data_dir() -> Result<PathBuf, io::Error> {
    if let Some(path) = env::var_os(DATA_DIR_ENV) {
        return Ok(PathBuf::from(path));
    }

    ProjectDirs::from("io", "xshaduwulfx", "Fenrir")
    .map(|dirs| dirs.data_dir().to_path_buf())
    .ok_or_else(|| {
        io::Error::other("Unable to determine the application data directory")
    })
}
