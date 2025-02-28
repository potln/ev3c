use super::Arguments; // Needed for processing the arguments object.
use crate::error::{Error, ErrorKind};

use std::fs::canonicalize;

/// Check all compiler options
/// before the compiler is initiated.
pub fn check(arguments: Arguments) -> Result<Arguments, Error> {
    // Loop over each file in the provided file list.
    for file in &arguments.files {
        // Convert to a path
        let path = file.as_path();

        // Check if each file exists
        if !path.exists() || !path.is_file() {
            return Err(Error::new("", ErrorKind::FileError));
        }
    }

    // Loop over each file in the provided include list.
    for file in &arguments.include {
        // Convert to a path
        let path = file.as_path();

        // Check if each file exists
        if !path.exists() || !path.is_file() {
            return Err(Error::new("", ErrorKind::FileError));
        }
    }

    // Check if the directory of the target file exists.
    let parent = arguments.options.target.parent().expect("Expected");
    let path = canonicalize(parent);

    // Check if the parent directory exists.
    if !path.is_ok() || !path.unwrap().exists() {
        return Err(Error::new("", ErrorKind::FileError));
    }

    // No errors were detected.
    return Ok(arguments);
}
