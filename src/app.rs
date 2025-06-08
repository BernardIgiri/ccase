use crate::{UserCase, UserPattern};
use clap::{builder::StyledStr, crate_version, Arg, ArgAction, Command};
use std::fmt::Write;
use strum::IntoEnumIterator;

pub fn build() -> Command {
    Command::new("ccase")
        .version(crate_version!())
        .about("Convert between string cases.\nhttps://github.com/rutrum/ccase")
        .arg_required_else_help(true)
        .args(args::all())
        .override_usage(usage())
        .max_term_width(90)
        .after_help(after_help())
        .after_long_help(after_long_help())
}

fn usage() -> StyledStr {
    StyledStr::from(
        "\x1b[1mccase --to\x1b[0m <case> <input>...\n       \
         \x1b[1mccase --to\x1b[0m <case> \x1b[1m--from\x1b[0m <case> <input>...",
    )
}

fn after_long_help() -> StyledStr {
    let s = format!(
        "\x1b[1;4mCase Conversion:\x1b[0m\n  \
        Visit the code repository at https://github.com/rutrum/ccase for a full description\n  \
        on how strings are converted using boundaries, patterns, and delimeters.\n\n\
        \x1b[1;4mCases:\x1b[0m\n\
        {}\n\
        \x1b[1;4mPatterns:\x1b[0m\n\
        {}\n\
        ",
        list_cases(),
        list_patterns(),
    );

    StyledStr::from(s)
}

fn after_help() -> StyledStr {
    StyledStr::from(
        "\x1b[1;4mCases:\x1b[0m\n  See --help for list of cases
    ",
    )
}

fn list_cases() -> String {
    let mut s = String::new();
    for case in UserCase::iter() {
        let case_str = case.to_string();
        let underline_case = format!("\x1b[1m{}\x1b[0m", case_str);
        writeln!(s, "{:>25}  {}", underline_case, case.example()).unwrap();
    }
    s
}

fn list_patterns() -> String {
    let mut s = String::new();
    for pattern in UserPattern::iter() {
        let pattern_str = pattern.to_string();
        let underline_pattern = format!("\x1b[1m{}\x1b[0m", pattern_str);
        writeln!(s, "{:>25}  {}", underline_pattern, pattern.example()).unwrap();
    }
    s
}

mod args {
    use clap::value_parser;

    use super::*;

    pub fn all() -> [Arg; 6] {
        [to(), from(), input(), boundaries(), pattern(), delimeter()]
    }

    fn to() -> Arg {
        Arg::new("to")
            .short('t')
            .long("to")
            .value_name("case")
            .help("Case to convert to")
            .long_help(
                "Convert the input into this case.  \
                The input is mutated and joined using the pattern and delimiter of the case.",
            )
            .value_parser(value_parser!(UserCase))
            .required_unless_present("pattern")
    }

    fn from() -> Arg {
        Arg::new("from")
            .short('f')
            .long("from")
            .value_name("case")
            .help("Case to parse input as")
            .long_help(
                "Parse the input as if it were this case.  \
                This means splitting the input based on boundaries found in that case.",
            )
            .value_parser(value_parser!(UserCase))
    }

    fn boundaries() -> Arg {
        Arg::new("boundaries")
            .short('b')
            .long("boundaries")
            .value_name("string")
            .help("String of boundaries to split input")
            .long_help(
                "String that contains boundaries on how to split input.  \
                Any boundary contained in the string will be used as boundaries \
                for splitting input into words.",
            )
            .conflicts_with("from")
    }

    fn pattern() -> Arg {
        Arg::new("pattern")
            .short('p')
            .long("pattern")
            .help("Pattern to transform words")
            .long_help("Transform the words after splitting the input based upon the pattern.")
            .conflicts_with("to")
            .value_parser(value_parser!(UserPattern))
    }

    fn delimeter() -> Arg {
        Arg::new("delimeter")
            .short('d')
            .long("delimeter")
            .help("String to join words by")
            .long_help("String to join words together after transforming.")
            .conflicts_with("to")
            .value_name("string")
            .allow_hyphen_values(true)
    }

    fn input() -> Arg {
        Arg::new("input")
            .help("The string(s) to convert")
            .long_help("The string(s) to convert into the --to case.")
            .action(ArgAction::Append)
    }
}
