use std::path::PathBuf;

const MAX_SEARCH_DEPTH: usize = 10;
const SYSTEM_PATHS: &[&str] = &[
    // Common
    "/",
    "/tmp",
    "/var",
    "/usr",
    "/opt",
    // macOS specific
    "/System",
    "/Users",
    "/Applications",
    "/Library",
    "/System/Library",
    "/Volumes",
    "/private",
    // Linux specific
    "/proc",
    "/sys",
    "/dev",
    "/boot",
    "/mnt",
    "/run",
    "/media",
    // Windows specific (using forward slashes for consistency)
    "C:/Windows",
    "C:/Program Files",
    "C:/Program Files (x86)",
    "C:/ProgramData",
    "C:/System32",
];

/// Find project root by scanning upward for prioritized indicators
pub fn get_root(current_dir: PathBuf) -> Result<PathBuf, String> {
    // Validate that the directory exists
    if !current_dir.exists() {
        return Err("Invalid current directory".to_string());
    }
    if !current_dir.is_dir() {
        return Err("Invalid current directory".to_string());
    }

    let dir = current_dir;

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
        let mut level = 0;
        loop {
            // Skip filesystem root
            if *check_dir == *"/" {
                break;
            }

            // Skip if exceeded max search depth
            if level > MAX_SEARCH_DEPTH {
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

                let normalized_check = check_dir.to_string_lossy().replace('\\', "/");
                if exists && !SYSTEM_PATHS.iter().any(|&s| s == normalized_check) {
                    return Ok(check_dir);
                }
            }

            // Ascend to parent
            match check_dir.parent() {
                Some(parent) => check_dir = parent.to_path_buf(),
                None => break,
            }
            level += 1;
        }
    }

    Err("No project root found (no .git, package file, or project dir)".to_string())
}

pub fn get_root_no_param() -> Result<PathBuf, String> {
    let curr_dir = std::env::current_dir().map_err(|e| format!("Failed to get root: {}", e))?;
    get_root(curr_dir)
}
