use colored::Colorize;
use fancy_regex::Regex;
use std::{fs, path::PathBuf};

/// Searches text for lines containing a regex query
/// and returns a vector of line number and match tuples.
///
/// # Examples
/// ```
/// use fancy_regex::Regex;
/// use grepme::search;
///
/// let query = Regex::new(r"(?i)DuCk").unwrap();
/// let contents = "\
/// rust? what is that
/// C++ works just as well
/// so you can duck off.";
///
/// assert_eq!(
///     vec![(2 as usize, "so you can duck off.".to_string())],
///     search::get_matches(&query, contents)
/// );
/// ```
pub fn get_matches(query: &Regex, contents: &str) -> Vec<(usize, String)> {
    let mut matches: Vec<(usize, String)> = Vec::new();
    for (index, content) in contents.lines().enumerate() {
        if query.is_match(content).unwrap_or(false) {
            matches.push((index, content.to_string()));
        }
    }
    matches
}

/// Searches every file in `paths` for matches against `query`, returning the
/// formatted, colorized output along with a count of files skipped due to
/// invalid UTF-8.
pub fn search_valid_files(
    paths: &[PathBuf],
    query: &fancy_regex::Regex,
) -> (String, usize) {
    let mut output = String::new();
    let mut skipped_count: usize = 0;

    for file_path in paths {
        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => {
                skipped_count += 1;
                continue;
            }
        };

        let results: Vec<(usize, String)> = get_matches(query, &contents);

        if !results.is_empty() {
            for (index, result) in results {
                output.push_str(&format!(
                    "{}:{} > {}\n",
                    file_path.display().to_string().bold().cyan(),
                    (index + 1).to_string().bold().green(),
                    result.italic()
                ));
            }
            output.push('\n');
        }
    }

    (output, skipped_count)
}
