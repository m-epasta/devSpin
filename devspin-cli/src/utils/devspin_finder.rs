use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use walkdir::WalkDir;

/// Find devspin.yml using parallel search with Rayon
pub fn find_devspin_yml_parallel(root: impl AsRef<Path>) -> Result<PathBuf, String> {
    find_devspin_yml_with_timeout(root, Duration::from_secs(30))
}

/// Version with timeout
pub fn find_devspin_yml_with_timeout(
    root: impl AsRef<Path>,
    timeout: Duration,
) -> Result<PathBuf, String> {
    let start = Instant::now();

    // Quick root check first (common case optimization)
    let root_path = root.as_ref();
    let root_file = root_path.join("devspin.yml");
    if root_file.is_file() {
        println!("Found in root after {:?}", start.elapsed());
        return Ok(root_file);
    }

    // Validate root exists
    if !root_path.exists() {
        return Err(format!(
            "Root directory '{}' does not exist",
            root.as_ref().display()
        ));
    }

    if !root_path.is_dir() {
        return Err(format!("'{}' is not a directory", root.as_ref().display()));
    }

    println!("Starting parallel search in: {}", root.as_ref().display());

    // Single parallel search with filter_entry
    let result = WalkDir::new(&root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|entry| !should_skip_entry(entry))
        .par_bridge()
        .filter_map(|entry| {
            // Check timeout periodically
            if start.elapsed() > timeout {
                return None;
            }
            entry.ok()
        })
        .find_any(|entry| {
            entry.file_type().is_file()
                && ["devspin.yml", "devspin.yaml", ".devspin.yml"]
                    .iter()
                    .any(|&name| entry.file_name() == name)
        })
        .map(|entry| entry.path().to_path_buf());

    match result {
        Some(path) => {
            println!("Found in {:?}: {}", start.elapsed(), path.display());
            Ok(path)
        }
        None => {
            let elapsed = start.elapsed();
            if elapsed > timeout {
                Err(format!(
                    "Search timed out after {:?} in '{}'",
                    timeout,
                    root.as_ref().display()
                ))
            } else {
                println!("Not found after {:?}", elapsed);
                Err(format!(
                    "devspin.yml not found under '{}'",
                    root.as_ref().display()
                ))
            }
        }
    }
}

/// Check if we should skip this entry (and its children)
fn should_skip_entry(entry: &walkdir::DirEntry) -> bool {
    let skip_dirs: [&str; 51] = [
        // Node/JavaScript
        "node_modules",
        "npm",
        ".npm",
        ".yarn",
        "yarn-cache",
        // Rust
        "target",
        "dist",
        "build",
        ".cargo",
        // Python
        "__pycache__",
        "venv",
        "env",
        ".venv",
        ".env",
        "pip",
        ".pytest_cache",
        ".mypy_cache",
        // Version Control
        ".git",
        ".svn",
        ".hg",
        ".bzr",
        // IDEs
        ".idea",
        ".vscode",
        ".vs",
        ".history",
        // Build outputs
        "_build",
        "out",
        "output",
        "release",
        "debug",
        "bin",
        "obj",
        "packages",
        // Dependencies
        "vendor",
        "deps",
        "dependencies",
        "third_party",
        // Cache
        ".cache",
        "cache",
        "tmp",
        "temp",
        ".tmp",
        ".temp",
        // Coverage
        "coverage",
        ".coverage",
        "htmlcov",
        // Logs
        "logs",
        "log",
        ".log",
        // OS
        ".DS_Store",
        "Thumbs.db",
    ];

    if entry.file_type().is_dir() {
        let name = entry.file_name().to_string_lossy().to_lowercase();
        skip_dirs.contains(&name.as_str())
    } else {
        false
    }
}
