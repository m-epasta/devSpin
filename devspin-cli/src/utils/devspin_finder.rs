use rayon::prelude::*;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use std::time::{Instant, Duration};

/// Find devspin.yml using parallel search with Rayon
pub fn find_devspin_yml_parallel(root: &str) -> Result<String, String> {
    find_devspin_yml_with_timeout(root, Duration::from_secs(30))
}

/// Version with timeout
pub fn find_devspin_yml_with_timeout(root: &str, timeout: Duration) -> Result<String, String> {
    let start = Instant::now();
    
    // Quick root check first (common case optimization)
    let root_path = Path::new(root);
    let root_file = root_path.join("devspin.yml");
    if root_file.is_file() {
        println!("Found in root after {:?}", start.elapsed());
        return Ok(root_file.to_string_lossy().into_owned());
    }
    
    // Validate root exists
    if !root_path.exists() {
        return Err(format!("Root directory '{}' does not exist", root));
    }
    
    if !root_path.is_dir() {
        return Err(format!("'{}' is not a directory", root));
    }
    
    println!("Starting parallel search in: {}", root);
    
    // Single parallel search with filter_entry
    let result = WalkDir::new(root)
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
                && entry.file_name() == "devspin.yml"
                && entry.path().extension().map(|e| e == "yml").unwrap_or(false)
        })
        .map(|entry| entry.path().to_string_lossy().into_owned());
    
    match result {
        Some(path) => {
            println!("Found in {:?}: {}", start.elapsed(), path);
            Ok(path)
        },
        None => {
            let elapsed = start.elapsed();
            if elapsed > timeout {
                Err(format!("Search timed out after {:?} in '{}'", timeout, root))
            } else {
                println!("Not found after {:?}", elapsed);
                Err(format!("devspin.yml not found under '{}'", root))
            }
        }
    }
}

/// Check if we should skip this entry (and its children)
fn should_skip_entry(entry: &walkdir::DirEntry) -> bool {
    let skip_dirs = [
        // Node/JavaScript
        "node_modules", "npm", ".npm", ".yarn", "yarn-cache",
        // Rust
        "target", "dist", "build", ".cargo", 
        // Python
        "__pycache__", "venv", "env", ".venv", ".env",
        "pip", ".pytest_cache", ".mypy_cache",
        // Version Control
        ".git", ".svn", ".hg", ".bzr",
        // IDEs
        ".idea", ".vscode", ".vs", ".history",
        // Build outputs
        "_build", "out", "output", "release", "debug",
        "bin", "obj", "packages",
        // Dependencies
        "vendor", "deps", "dependencies", "third_party",
        // Cache
        ".cache", "cache", "tmp", "temp", ".tmp", ".temp",
        // Coverage
        "coverage", ".coverage", "htmlcov",
        // Logs
        "logs", "log", ".log",
        // OS
        ".DS_Store", "Thumbs.db",
    ];
    
    if entry.file_type().is_dir() {
        let name = entry.file_name().to_string_lossy().to_lowercase();
        skip_dirs.contains(&name.as_str())
    } else {
        false
    }
}

/// Search multiple possible config names
pub fn find_config_file(root: &str) -> Result<PathBuf, String> {
    let config_names = ["devspin.yml", "devspin.yaml", ".devspin.yml"];
    
    for name in &config_names {
        let path = Path::new(root).join(name);
        if path.is_file() {
            return Ok(path);
        }
    }
    
    // Fall back to parallel search
    let result = WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !should_skip_entry(entry))
        .par_bridge()
        .filter_map(|e| e.ok())
        .find_any(|entry| {
            entry.file_type().is_file() && 
            config_names.iter().any(|&name| entry.file_name() == name)
        })
        .map(|e| e.path().to_path_buf());
    
    result.ok_or_else(|| {
        let names = config_names.join(", ");
        format!("None of [{}] found under '{}'", names, root)
    })
}
// TODO: verify the code because im getting hrut by my mom