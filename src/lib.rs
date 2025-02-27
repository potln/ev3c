use std::path::PathBuf;

pub mod args;
pub mod parser;

/// Specifies which level of optimizations to use,
/// note that levels "High" and "Size" may be unstable.
#[derive(Debug)]
pub enum OptimizationLevel {
    None,   // -O0
    Low,    // -O1
    Medium, // -O2
    High,   // -O3
    Size,   // -Oz
}

/// List of flags that enable specific
/// warnings within the compilation process.
#[derive(Debug)]
pub enum WarningFlags {
    All,
    None,
}

/// List of compiler options.
/// Intended for use in the compiler, not in the library.
#[derive(Debug)]
pub struct Options {
    /// File used to output the compiled result.
    /// If multiple files are used they will be
    /// combined into a single file.
    target: PathBuf,

    /// Level of optimization, used within the
    /// compiler to enable certain optimizations.
    optimization: OptimizationLevel,

    /// List of warnings to be enabled
    /// during the compilation process, 'All'
    /// enables every warning message.
    warnings: Vec<WarningFlags>,
}

// Allow for the instantiation of a default
// options struct.
impl Options {
    /// Create a new options instance.
    fn new() -> Options {
        return Options {
            target: PathBuf::from("a.rbf"), // Following compiler standard with an rbf file.
            optimization: OptimizationLevel::Low, // Set optimizations to the most stable.
            warnings: vec![WarningFlags::All], // Enable basic warnings.
        };
    }
}
