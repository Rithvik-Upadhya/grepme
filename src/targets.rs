//! Expands a list of file/directory arguments into a flat list of files
//! that should be searched.
use std::{fs, path::PathBuf};

/// Represents a non-fatal problem encountered while expanding targets into
/// a list of files. These are collected and reported, but do not stop the
/// rest of the expansion from proceeding.
pub enum ExpansionFailed {
    /// An I/O error occurred while reading a path's metadata or directory
    /// entries. Contains the offending path and the error message.
    IoError((PathBuf, String)),
    /// A symlink was encountered and was skipped rather than followed.
    SymlinkNotFollowed(PathBuf),
    /// A directory could not be read (e.g. due to permissions).
    FolderNotRecursed(PathBuf),
}

#[cfg(unix)]
fn get_ids(metadata: &fs::Metadata) -> (u64, u64) {
    use std::os::unix::fs::MetadataExt;
    (metadata.dev(), metadata.ino())
}

/// Expands a list of target paths into a flat list of files to search.
///
/// Files are included as-is; directories are recursed into (non-recursively
/// skipping symlinks); and, if `output_file` is provided, that exact file is
/// excluded from the results (only on unix systems) so a search doesn't read
/// from the file it's about to write to.
///
/// Returns a tuple of the collected file paths and any non-fatal errors
/// encountered along the way.
pub fn expand_to_files(
    targets: &Vec<PathBuf>,
    output_file: &Option<PathBuf>,
) -> (Vec<PathBuf>, Vec<ExpansionFailed>) {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut errors: Vec<ExpansionFailed> = Vec::new();

    #[cfg(unix)]
    let output_file_ids: Option<(u64, u64)> = output_file
        .as_ref()
        .and_then(|p| fs::metadata(p).ok())
        .map(|m| get_ids(&m));

    #[cfg(not(unix))]
    let output_file_ids: Option<(u64, u64)> = None;

    for target in targets {
        collect_files(target, &output_file_ids, &mut file_paths, &mut errors);
    }

    (file_paths, errors)
}

fn collect_files(
    target: &PathBuf,
    output_file_ids: &Option<(u64, u64)>,
    file_paths: &mut Vec<PathBuf>,
    errors: &mut Vec<ExpansionFailed>,
) {
    // Check if path exists
    let metadata = match target.metadata() {
        Ok(m) => m,
        Err(e) => {
            // file does not exist
            errors.push(ExpansionFailed::IoError((
                target.clone(),
                e.to_string(),
            )));
            return;
        }
    };

    #[cfg(unix)]
    if output_file_ids.is_some() {
        let target_ids = get_ids(&metadata);
        if target_ids == *output_file_ids.as_ref().unwrap() {
            // target is the output file itself, skip it
            return;
        }
    }

    if metadata.is_symlink() {
        // do not follow symlinks
        errors.push(ExpansionFailed::SymlinkNotFollowed(target.clone()));
    } else if metadata.is_file() {
        file_paths.push(target.clone());
    } else if metadata.is_dir() {
        let dir_entries = match fs::read_dir(target) {
            Ok(ds) => ds,
            Err(_) => {
                errors.push(ExpansionFailed::FolderNotRecursed(target.clone()));
                return;
            }
        };
        for dir_entry in dir_entries {
            let dir_entry = match dir_entry {
                Ok(d) => d,
                Err(e) => {
                    errors.push(ExpansionFailed::IoError((
                        target.clone(),
                        e.to_string(),
                    )));
                    return;
                }
            };
            collect_files(
                &dir_entry.path(),
                output_file_ids,
                file_paths,
                errors,
            );
        }
    }
}
