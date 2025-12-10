use std::path::PathBuf;

/// Find project root by scanning upward for prioritized indicators
pub fn get_root(current_dir: PathBuf) -> Result<PathBuf, String> {
    let mut dir = current_dir
        .canonicalize()
        .map_err(|_| "Invalid current directory".to_string())?;

    // Indicators as (name, is_directory) tuples - groups filesystem calls by type
    let indicators: &[(&str, bool)] = &[
        // Version Control (highest priority - most reliable)
        (".git", true), // Git repository
        (".svn", true), // Subversion
        (".hg", true),  // Mercurial
        (".bzr", true), // Bazaar
        // Package Managers & Configs
        ("Cargo.toml", false),       // Rust
        ("package.json", false),     // Node.js/JavaScript
        ("requirements.txt", false), // Python pip
        ("setup.py", false),         // Python setuptools
        ("pyproject.toml", false),   // Modern Python
        ("Gemfile", false),          // Ruby/Bundler
        ("composer.json", false),    // PHP/Composer
        ("go.mod", false),           // Go
        ("pom.xml", false),          // Java/Maven
        ("build.gradle", false),     // Java/Kotlin/Gradle
        // IDE & Editor configs
        (".vscode", true), // VS Code
        (".idea", true),   // IntelliJ IDEA
        // CI/CD & Repo
        (".github", true),         // GitHub Actions
        (".circleci", true),       // CircleCI
        (".travis.yml", false),    // Travis CI
        (".gitlab-ci.yml", false), // GitLab CI
        // Build tools
        ("Makefile", false),       // GNU Make
        ("CMakeLists.txt", false), // CMake
        ("Dockerfile", false),     // Docker
        ("Vagrantfile", false),    // Vagrant
        // Common project directories
        ("src", true),      // Source code
        ("lib", true),      // Libraries
        ("bin", true),      // Binaries/executables
        ("scripts", true),  // Scripts
        ("assets", true),   // Static assets
        ("public", true),   // Public files
        ("static", true),   // Static files
        ("docs", true),     // Documentation
        ("config", true),   // Configuration
        ("test", true),     // Tests (directory)
        ("tests", true),    // Tests (plural)
        ("examples", true), // Examples
    ];

    loop {
        // Check each indicator with appropriate fs call (dir vs file)
        for &(name, is_dir) in indicators {
            let path = dir.join(name);
            let exists = if is_dir {
                path.is_dir()
            } else {
                path.is_file()
            };

            if exists {
                return Ok(dir.clone());
            }
        }

        // Ascend to parent, stop at filesystem root
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => break,
        }
    }

    Err("No project root found (no .git, package file, or project dir)".to_string())
}

pub fn get_root_no_param() -> Result<PathBuf, String> {
    let curr_dir = std::env::current_dir().map_err(|e| format!("Failed to get root: {}", e))?;
    get_root(curr_dir)
}
