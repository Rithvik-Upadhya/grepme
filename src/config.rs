//! Command-line argument parsing (`Config`) and the CLI help text.
use std::{env, path::PathBuf};

/// Parsed command-line configuration for a `grepme` invocation.
pub struct Config {
    pub query: String,
    pub targets: Vec<PathBuf>,
    pub ignore_case: bool,
    pub output_file: Option<String>,
}

impl Config {
    /// Parses raw command-line arguments (as returned by [`std::env::args`])
    /// into a [`Config`].
    ///
    /// The first element of `args` is expected to be the executable path and
    /// is skipped. Any arguments starting with `-` are treated as flags and
    /// must appear before the query. The first non-flag argument is treated
    /// as the query, and everything after it as search targets (files or
    /// directories).
    ///
    /// # Examples
    /// ```
    /// use grepme::config::Config;
    ///
    /// let args: Vec<String> = vec![
    ///     "grepme".to_string(),
    ///     "-i".to_string(),
    ///     "duck".to_string(),
    ///     "src/main.rs".to_string(),
    /// ];
    ///
    /// let config = Config::build(&args).unwrap();
    ///
    /// assert_eq!(config.query, "duck");
    /// assert!(config.ignore_case);
    /// assert_eq!(config.targets, vec![std::path::PathBuf::from("src/main.rs")]);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Err` if too few arguments are supplied, an unrecognised
    /// flag is passed, or `--help`/`-h` is passed alongside other arguments.
    ///
    /// ```
    /// use grepme::config::Config;
    ///
    /// let args: Vec<String> = vec!["grepme".to_string()];
    /// assert!(Config::build(&args).is_err());
    /// ```
    pub fn build(args: &[String]) -> Result<Self, String> {
        let mut current_index: usize = 1; // 0 is the executable path called, so we start from 1
        if args.len() < current_index + 2 {
            return Err("not enough arguments".to_string());
        }
        let mut ignore_case: bool = match env::var("IGNORE_CASE") {
            Ok(val) => val.parse().unwrap_or(false),
            Err(_) => false,
        }; // can be overridden by flags
        let mut output_file: Option<String> = None;

        // get the command line args (if any)
        while args[current_index].starts_with('-') {
            let flag = args[current_index].clone();

            if flag.eq("-i") || flag.eq("--ignore-case") {
                ignore_case = true;
            } else if flag.eq("-o") || flag.eq("--output-file") {
                current_index += 1; // the next arg is the output file path
                output_file = Some(args[current_index].clone());
            } else if flag.eq("-h") || flag.eq("--help") {
                return Err(format!("the {flag} can only be used by itself."));
            } else {
                return Err(
                    "unrecogonised flag passed\nRun `grepme --help` to see available flags."
                        .to_string(),
                );
            }
            current_index += 1;
        }
        if args.len() < current_index + 2 {
            return Err(
                "query and file path are required arguments".to_string()
            );
        }
        let query: String = args[current_index].clone();
        current_index += 1;
        let targets: Vec<PathBuf> =
            args[current_index..].iter().map(PathBuf::from).collect();

        Ok(Self {
            query,
            targets,
            ignore_case,
            output_file,
        })
    }
}

/// The text printed for `grepme --help` / `grepme -h` (or when no arguments
/// are supplied).
pub const HELPTEXT: &str = "\
Usage: grepme [arguments] query [paths/glob]

Query can be any plain unquoted string or a quoted regex pattern.

Arguments:

--help / -h\tShows this help text
--ignore-case / -i\tPerforms case-insensitive search
--output-file / -o <filepath>\tWrites stdout to file";
