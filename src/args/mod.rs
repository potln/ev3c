use crate::OptimizationLevel; // Needed for specifying the level of optimization
use crate::Options; // Needed for the options section of the Arguments struct
use crate::WarningFlags; // Needed for adding warnings flags to the options struct

use std::env::Args; // Needed for reading in arguments
use std::path::Path; // Needed for the path section of the Arguments struct

/// Differentiate between different options
/// when parsing out leaders and their values.
enum OptionType {
    /// The option provided is unrecognized,
    /// and will throw a warning.
    Unknown,

    // Actual Options
    Target,
    Include,
    Optimization,

    // Warning Options
    WarningAll,
    WarningNone,
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

        // Match the OptionType to the method it will
        // will use to be further parsed.
        match leader.unwrap() {
            OptionType::Unknown => (),
            OptionType::Target | OptionType::Include => {
                add_option(&mut arguments, scanner.next(), leader.upwrap());
            }
            OptionType::Optimization => add_optimization(&mut arguments, &leader),
            OptionType::WarningAll | OptionType::WarningNone => {
                add_warning(&mut arguments, leader.unwrap());
            }
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
        return parse_warning(&leader[1..]);
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
    if leader.starts_with("warn-") {
        return parse_warning(&leader[5..]);
    }

    // Match each value to the option type.
    return Some(match leader {
        "output" => OptionType::Target,
        "include" => OptionType::Include,
        _ => OptionType::Unknown,
    });
}

/// Parse out the OptionType from the
/// given warnings string, which will eventually
/// be converted into a WarningFlag.
fn parse_warning(leader: &str) -> Option<OptionType> {
    // Match each value to the option type.
    return Some(match leader {
        "all" => OptionType::WarningAll,
        "none" => OptionType::WarningNone,
        _ => OptionType::Unknown,
    });
}

/// Add a warning option to the
/// arguments structure, resolving the provided
/// warning into the options object.
fn add_warning(arguments: &mut Arguments, warning_type: OptionType) {
    // Resolve the warning value from the warning type
    let warning = match warning_type {
        OptionType::WarningAll => WarningFlags::All,
        OptionType::WarningNone => WarningFlags::None,
        _ => panic!("Unexpected value found when processing warnings!"),
    };

    // Add the warning to the compiler options.
    arguments.options.warnings.push(warning);
}

fn add_option(arguments: &mut Arguments, value: String, option_type: OptionType) {
    match option_type {
        _ => panic!("Unexpected value found when processing options!"),
    };
}

/// Add an optimization level option to the
/// arguments structure, resolving the provided string
/// into it's proper optimization level.
fn add_optimization(arguments: &mut Arguments, leader: &str) {
    // Resolve the optimization level from the string.
    arguments.options.optimization = match leader {
        "O0" => OptimizationLevel::None,
        "O1" => OptimizationLevel::Low,
        "O2" => OptimizationLevel::Medium,
        "O3" => OptimizationLevel::High,
        "Oz" => OptimizationLevel::Size,
        _ => panic!("Unexpected value found when processing optimization level!"),
    }
}
