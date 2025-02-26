use crate::args::Arguments;
use crate::parser::lexer::scanner::Scanner; //
use crate::Options; // Needed for the options section of the Arguments struct

use std::env::Args; // Needed for reading in arguments
use std::path::Path; // Needed for the path section of the Arguments struc

/// Differentiate between different options
/// when parsing out leaders and their values.
enum OptionType {
    /// The option provided is unrecognized,
    /// and will throw a warning.
    Unknown,

    // Actual options
    Target,
    Include,
    Optimization,
}

/// Combined output of all parsed arguments,
/// used to guide the compiler.
pub struct Arguments {
    /// List of files to compile.
    /// Multiple different base files can be
    /// compilted at the same time.
    files: Vec<Path>,

    /// List of files to include,
    /// such as libraries and other external code.
    include: Vec<Path>,

    /// List of compiler options,
    /// parsed and converted to the
    /// proper data types.
    options: Options,
}

// Allow for the instatiation of a default
// Arguments struct.
impl Arguments {
    // Create a new instance of arguments
    fn new() -> Arguments {
        return Arguments {
            files: Vec::new(),       // A file will be required from the user.
            include: Vec::new(),     // No inclusions by default.
            options: Options::new(), // Default options.
        };
    }
}

/// Parse arguments into a combined structure
/// containing options and files.
pub fn parse(args: Args) -> Arguments {
    // Scan over the provided arguments
    let mut scanner = args.peekable();
    let mut arguments = Arguments::new();
    let mut files = Vec::new();

    // Loop over every string in the list of
    // arguments.
    while let Some(arg) = scanner.next() {
        // Check if the argument has a leader.
        let leader = parse_leader(arg);

        // Push directly to the list of files
        // if the argument does not have a leader.
        if leader.is_none() {
            files.push(Path::from(&arg));
            continue;
        }
    }

    return arguments;
}

/// Check if the provided argument has a
/// leader, and process it if it does.
fn parse_leader(arg: String) -> Option<OptionType> {
    // Check for a leader
    if !arg.starts_with("-") {
        return None();
    }

    // Remove leading '-'
    let leader = arg[1..];

    // Check if the argument is a double dash '--'
    if leader.starts_with("-") {
        return parse_expanded(&leader);
    }

    // Handle warnings
    if leader.starts_with("W") {
        return parse_warning(&leader);
    }

    // Match each value to the option type.
    return Some(match leader {
        "o" => OptionType::Target,
        "i" => OptionType::Include,
        "O0" | "O1" | "O2" | "O3" | "Oz" => OptionType::Optimization,
        _ => OptionType::Unknown,
    });
}

/// If the option name is expanded,
/// resolve it to be parsed.
fn parse_expanded(leader: &str) -> Option<OptionType> {
    // Handle warnings
    if leader.starts_with("warn-") {}

    // Match each value to the option type.
    return Some(match leader {
        "output" => OptionType::Target,
        "include" => OptionType::Include,
        _ => OptionType::Unknown,
    });
}

fn parse_warning(leader: &str) -> Option<OptionType> {
    // Match each value to the option type.
    return Some(match leader {
        "Wall" => OptionType::Warning,
        "Wextra" => OptionType::Warning,
        "Werror" => OptionType::Warning,
        _ => OptionType::Unknown,
    });
}
