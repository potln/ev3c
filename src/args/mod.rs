use crate::OptimizationLevel; // Needed for specifying the level of optimization
use crate::Options; // Needed for the options section of the Arguments struct
use crate::WarningFlags; // Needed for adding warnings flags to the options struct

mod info;

use std::env::Args; // Needed for reading in arguments
use std::path::PathBuf; // Needed for the path section of the Arguments struct

/// Differentiate between different options
/// when parsing out leaders and their values.
enum OptionType {
    /// The option provided is unrecognized,
    /// and will throw a warning.
    Unknown,

    // Info Options
    Help,
    Version,

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
#[derive(Debug)]
pub struct Arguments {
    /// List of files to compile.
    /// Multiple different base files can be
    /// compilted at the same time.
    pub files: Vec<PathBuf>,

    /// List of files to include,
    /// such as libraries and other external code.
    pub include: Vec<PathBuf>,

    /// List of compiler options,
    /// parsed and converted to the
    /// proper data types.
    pub options: Options,
}

// Allow for the instatiation of a default
// Arguments struct.
impl Arguments {
    // Create a new instance of arguments
    pub fn new() -> Arguments {
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

    // Skip over the executable call.
    scanner.next();

    // Loop over every string in the list of
    // arguments.
    while let Some(arg) = scanner.next() {
        // Check if the argument has a leader.
        let leader = parse_leader(&arg);

        // Push directly to the list of files
        // if the argument does not have a leader.
        if leader.is_none() {
            arguments.files.push(PathBuf::from(&arg));
            continue;
        }

        // Unwrap the leader.
        let leader = leader.unwrap();

        // Match the OptionType to the method it will
        // will use to be further parsed.
        match leader {
            OptionType::Unknown => (),
            OptionType::Target | OptionType::Include => {
                add_option(
                    &mut arguments,
                    scanner.next().expect("Expected value for option!"),
                    leader,
                );
            }
            OptionType::Optimization => add_optimization(&mut arguments, &arg[1..]),
            OptionType::WarningAll | OptionType::WarningNone => {
                add_warning(&mut arguments, leader);
            }
            OptionType::Help => println!("{}", info::help_message()),
            OptionType::Version => println!("{}", info::version_message()),
        }
    }

    return arguments;
}

/// Check if the provided argument has a
/// leader, and process it if it does.
fn parse_leader(arg: &str) -> Option<OptionType> {
    // Check for a leader
    if !arg.starts_with("-") {
        return None;
    }

    // Remove leading '-'
    let leader = &arg[1..];

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
        "h" => OptionType::Help,
        "v" => OptionType::Version,
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
        "help" => OptionType::Help,
        "version" => OptionType::Version,
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

/// Parse out a list of values into
/// a list of paths.
fn parse_path_list(value: &str) -> Vec<PathBuf> {
    let mut list = Vec::new();
    let mut scanner = value.chars().peekable();

    while scanner.peek().is_some() {
        // Create a string to store each path.
        let mut item = String::new();

        // Collect values until ',' is found or the text is consumed.
        while let Some(character) = scanner.next() {
            // Check if the character is a seperator (',')
            if character == ',' {
                break;
            }

            // Collect each character.
            item.push(character);
        }

        // Add the finished path to the list of paths.
        list.push(PathBuf::from(item));
    }

    return list;
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

/// Add an option to the list of arguments
/// based on its Optiontype and value.
fn add_option(arguments: &mut Arguments, value: String, option_type: OptionType) {
    match option_type {
        OptionType::Target => arguments.options.target = PathBuf::from(value),
        OptionType::Include => arguments.include.append(&mut parse_path_list(&value)),
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
        _ => panic!(
            "Unexpected value found when processing optimization level! ({})",
            leader
        ),
    }
}
