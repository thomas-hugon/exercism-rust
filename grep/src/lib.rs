use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Default)]
pub struct Flags {
    print_line: bool,
    print_only_filename: bool,
    case_insensitive: bool,
    invert_search: bool,
    match_entire_lines: bool,
}

impl Flags {
    pub fn new(flags_str: &[&str]) -> Self {
        let mut flags = Flags::default();
        for flag_str in flags_str {
            match *flag_str {
                "-n" => flags.print_line = true,
                "-l" => flags.print_only_filename = true,
                "-i" => flags.case_insensitive = true,
                "-v" => flags.invert_search = true,
                "-x" => flags.match_entire_lines = true,
                _ => {}
            }
        }
        flags
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let string_comparator = match flags.match_entire_lines{
        true => |(text, pattern)| text == pattern,
        _ => |(text, pattern): (String, String)| text.contains(&pattern)
    };
    let string_builder = match flags.case_insensitive {
        true => |text: &str, pattern: &str| (text.to_lowercase(), pattern.to_lowercase()),
        _ => |text: &str, pattern: &str| (text.to_string(), pattern.to_string())
    };
    let match_inverter = match flags.invert_search {
        true => |matches: bool| !matches,
        _ => |matches| matches
    };

    let format = match (flags.print_only_filename, files.len() > 1, flags.print_line) {
        (true, _, _) => |_, file_name: &str, _| file_name.to_string(),
        (false, true, true) => |i: usize, file_name: &str, line: String| format!("{}:{}:{}", file_name, i, line),
        (false, true, false) => |_, file_name: &str, line: String| format!("{}:{}", file_name, line),
        (false, false, true) => |i: usize, _: &str, line: String| format!("{}:{}", i, line),
        (false, false, false) => |_, _: &str, line: String| line,
    };

    let mut vec = Vec::new();
    for filename in files {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        for (i, line) in (1usize..).zip(reader.lines()) {
            let line = line?;
            if match_inverter(string_comparator(string_builder(&line, pattern))) {
                vec.push(format(i, filename, line));
                if flags.print_only_filename {
                    break;
                }
            }
        }
    }
    Ok(vec)
}
