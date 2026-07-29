use colored::{Colorize, control};
use fancy_regex::RegexBuilder;
use std::{env, error::Error, fs, path::PathBuf, process};

use grepme::config::{Config, HELPTEXT};
use grepme::{ExpansionFailed, search, targets};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && (args[1].eq("--help") || args[1].eq("-h")) {
        println!("{HELPTEXT}");
        process::exit(0);
    }

    if args.len() == 2 && (args[1].eq("--version") || args[1].eq("-v")) {
        println!(
            "{} {}{}",
            env!("CARGO_PKG_NAME"),
            "v".green().bold(),
            env!("CARGO_PKG_VERSION").green().bold(),
        );
        process::exit(0);
    }

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Problem getting matches: {e}");
        process::exit(1);
    }
}

/// Runs a search using the given configuration: builds the regex, expands
/// targets into files, searches each file, and prints/writes the results.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let output_file = config.output_file.map(PathBuf::from);
    if output_file.is_some() {
        // Turn off coloring when writing to an output file.
        control::set_override(false);
    }

    let query = build_query(&config.query, config.ignore_case)?;
    let (paths, errors) =
        targets::expand_to_files(&config.targets, &output_file);

    report_expansion_errors(&errors);

    let (output, skipped_count) = search::search_valid_files(&paths, &query);

    if skipped_count > 0 {
        eprintln!(
            "{} {} {}",
            "Skipped parsing".yellow().bold(),
            skipped_count.to_string().yellow().bold(),
            "files containing invalid UTF-8".yellow().bold(),
        );
    }

    write_output(&output, output_file)
}

/// Compiles the user's query string into a regex, optionally case-insensitive.
fn build_query(
    query: &str,
    ignore_case: bool,
) -> Result<fancy_regex::Regex, fancy_regex::Error> {
    let mut query_builder = RegexBuilder::new(query);
    if ignore_case {
        query_builder.case_insensitive(true).build()
    } else {
        query_builder.build()
    }
}

/// Prints any non-blocking issues encountered while expanding targets
/// (e.g. unreadable files, skipped symlinks) to stderr.
fn report_expansion_errors(errors: &[ExpansionFailed]) {
    if errors.is_empty() {
        return;
    }

    eprintln!("{}", "non-blocking issues:".red().bold());
    for error in errors {
        match error {
            ExpansionFailed::IoError((p, e)) => {
                eprintln!(
                    "    {} {} - {}",
                    "io error:".red(),
                    p.display(),
                    e.italic()
                )
            }
            ExpansionFailed::FolderNotRecursed(p) => {
                eprintln!(
                    "    {} {}",
                    "could not read folder:".red(),
                    p.display()
                )
            }
            ExpansionFailed::SymlinkNotFollowed(p) => {
                eprintln!("    {} {}", "symlink skipped:".red(), p.display())
            }
        }
    }
}

/// Writes the formatted search output either to stdout or to the given
/// output file, or reports that no matches were found.
fn write_output(
    output: &str,
    output_file: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    if output.is_empty() {
        eprintln!("No matches found");
        return Ok(());
    }

    match output_file {
        Some(path) => fs::write(path, output.trim_end_matches('\n'))?,
        None => println!("{}", output.trim_end_matches('\n')),
    }

    Ok(())
}
