/// Display help information
/// when the help option is enabled.
pub fn help_message() -> &'static str {
    return &"";
}

/// Display version information
/// when the version option is enabled.
pub fn version_message() -> &'static str {
    return &concat!("ev3c v", env!("CARGO_PKG_VERSION"));
}
