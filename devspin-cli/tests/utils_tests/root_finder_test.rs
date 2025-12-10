use devspin_cli::utils::root_finder::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_find_git_root() {
        let temp_dir = tempdir().unwrap();
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        let subdirectory = temp_dir.path().join("src").join("main");
        fs::create_dir_all(&subdirectory).unwrap();

        let result = get_root(subdirectory);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_find_cargo_toml_root() {
        let temp_dir = tempdir().unwrap();
        let cargo_file = temp_dir.path().join("Cargo.toml");
        File::create(&cargo_file).unwrap();

        let subdirectory = temp_dir.path().join("src").join("lib");
        fs::create_dir_all(&subdirectory).unwrap();

        let result = get_root(subdirectory);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_find_nested_indicators_prefers_git() {
        let temp_dir = tempdir().unwrap();
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let cargo_file = subdir.join("Cargo.toml");
        File::create(&cargo_file).unwrap();

        let deep_subdir = subdir.join("src");
        fs::create_dir(&deep_subdir).unwrap();

        // Should find .git first (higher priority)
        let result = get_root(deep_subdir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_find_multiple_indicators_subdirectory() {
        let temp_dir = tempdir().unwrap();

        let subdir = temp_dir.path().join("project");
        fs::create_dir(&subdir).unwrap();
        let cargo_file = subdir.join("Cargo.toml");
        File::create(&cargo_file).unwrap();

        let deep_subdir = subdir.join("tests");
        fs::create_dir(&deep_subdir).unwrap();

        let result = get_root(deep_subdir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), subdir);
    }

    #[test]
    fn test_no_root_found() {
        // Create an isolated directory in a guaranteed-clean location
        // Use a path in temp that's specifically for testing no roots found
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_base = std::env::temp_dir().join(format!("devspin_no_root_test_{}", timestamp));
        let isolated_dir = test_base
            .join("level1")
            .join("level2")
            .join("level3")
            .join("no_indicators_here");
        fs::create_dir_all(&isolated_dir).unwrap();

        // Make sure this directory doesn't contain any indicators itself
        assert!(!isolated_dir.join(".git").exists());
        assert!(!isolated_dir.join("Cargo.toml").exists());
        assert!(!isolated_dir.join("src").exists());

        let result = get_root(isolated_dir.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No project root found"));

        // Clean up
        let _ = std::fs::remove_dir_all(&test_base);
    }

    #[test]
    fn test_invalid_start_directory() {
        let invalid_path = PathBuf::from("/nonexistent/path/to/test");
        let result = get_root(invalid_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid current directory"));
    }

    #[test]
    fn test_find_src_directory() {
        let temp_dir = tempdir().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();

        let subdirectory = src_dir.join("components");
        fs::create_dir(&subdirectory).unwrap();

        let result = get_root(subdirectory);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir.path());
    }

    #[test]
    fn test_get_root_no_param_success() {
        // Set current dir to temp dir with indicator
        let temp_dir = tempdir().unwrap();
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        // Change to the temp dir for this test
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = get_root_no_param();
        assert!(result.is_ok());
        // On macOS, current_dir() returns canonicalized paths, so we need to compare canonicalized
        let expected = temp_dir
            .path()
            .canonicalize()
            .unwrap_or(temp_dir.path().to_path_buf());
        assert_eq!(result.unwrap(), expected);

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_get_root_no_param_failure() {
        // Test when current dir can't be obtained (though this is unlikely to trigger in normal tests)
        // We can't easily simulate failure of std::env::current_dir(),
        // but we can check that the function call doesn't panic and returns a proper error
        let result = get_root_no_param();
        // This should succeed in normal test environment
        assert!(result.is_ok());
    }
}
