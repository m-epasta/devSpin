use std::path::PathBuf;

/// Find project root by scanning upward for prioritized indicators
pub fn get_root(current_dir: PathBuf) -> Result<PathBuf, String> {
    let dir = current_dir
        .canonicalize()
        .map_err(|_| "Invalid current directory".to_string())?;

    // Indicators grouped by priority - search groups in order
    let indicator_groups: &[&[(&str, bool)]] = &[
        // Version Control (highest priority - most reliable)
        &[
            (".git", true),
            (".svn", true),
            (".hg", true),
            (".bzr", true),
        ],
        // Package Managers & Configs
        &[
            ("Cargo.toml", false),
            ("package.json", false),
            ("requirements.txt", false),
            ("setup.py", false),
            ("pyproject.toml", false),
            ("Gemfile", false),
            ("composer.json", false),
            ("go.mod", false),
            ("pom.xml", false),
            ("build.gradle", false),
        ],
        // IDE & Editor configs
        &[(".vscode", true), (".idea", true)],
        // CI/CD & Repo
        &[(".github", true), (".circleci", true)],
        // Build tools & files
        &[
            ("Makefile", false),
            ("CMakeLists.txt", false),
            ("Dockerfile", false),
            ("Vagrantfile", false),
            (".travis.yml", false),
            (".gitlab-ci.yml", false),
        ],
        // Common project directories
        &[
            ("src", true),
            ("lib", true),
            ("bin", true),
            ("scripts", true),
            ("assets", true),
            ("public", true),
            ("static", true),
            ("docs", true),
            ("config", true),
            ("test", true),
            ("tests", true),
            ("examples", true),
        ],
    ];

    // Search each group in priority order
    for &group in indicator_groups {
        let mut check_dir = dir.clone();
        loop {
            // Skip filesystem root
            if *check_dir == *"/" {
                break;
            }

            // Check if any indicator in this group exists at current level
            for &(name, is_dir) in group {
                let path = check_dir.join(name);
                let exists = if is_dir {
                    path.is_dir()
                } else {
                    path.is_file()
                };

                if exists {
                    return Ok(check_dir);
                }
            }

            // Ascend to parent
            match check_dir.parent() {
                Some(parent) => check_dir = parent.to_path_buf(),
                None => break,
            }
        }
    }

    Err("No project root found (no .git, package file, or project dir)".to_string())
}

pub fn get_root_no_param() -> Result<PathBuf, String> {
    let curr_dir = std::env::current_dir().map_err(|e| format!("Failed to get root: {}", e))?;
    get_root(curr_dir)
}
