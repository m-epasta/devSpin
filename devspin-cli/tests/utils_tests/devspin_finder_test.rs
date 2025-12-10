use devspin_cli::utils::devspin_finder::*;

#[cfg(test)]
/// Tests for devspin_finder utilities that search for devspin config files
/// Files tested: devspin.yml, devspin.yaml, .devspin.yml
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_finds_file_in_root() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("devspin.yml");
        File::create(&config_path).unwrap();

        let result = find_devspin_yml_parallel(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), config_path);
    }

    #[test]
    fn test_finds_file_in_subdirectory() {
        let temp_dir = tempdir().unwrap();
        let subdir = temp_dir.path().join("config");
        fs::create_dir(&subdir).unwrap();
        let config_path = subdir.join("devspin.yml");
        File::create(&config_path).unwrap();

        let result = find_devspin_yml_parallel(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), config_path);
    }

    #[test]
    fn test_finds_all_file_variants() {
        let file_variants = ["devspin.yml", "devspin.yaml", ".devspin.yml"];

        for variant in file_variants.iter() {
            let temp_dir = tempdir().unwrap();
            let config_path = temp_dir.path().join(variant);
            File::create(&config_path).unwrap();

            let result = find_devspin_yml_parallel(temp_dir.path().to_str().unwrap());
            assert!(result.is_ok(), "Failed to find {}", variant);
            assert_eq!(result.unwrap(), config_path);
        }
    }

    #[test]
    fn test_skips_common_directories() {
        let skip_dirs = ["node_modules", "target", ".git", ".vscode"];

        for dir in skip_dirs.iter() {
            let temp_dir = tempdir().unwrap();
            let skip_dir = temp_dir.path().join(dir);
            fs::create_dir(&skip_dir).unwrap();
            let config_path = skip_dir.join("devspin.yml");
            File::create(&config_path).unwrap();

            let result = find_devspin_yml_parallel(temp_dir.path().to_str().unwrap());
            assert!(result.is_err(), "Should NOT find devspin.yml in {}", dir);
        }
    }

    #[test]
    fn test_handles_missing_root() {
        let temp_dir = tempdir().unwrap();
        let nonexistent = temp_dir.path().join("nonexistent");
        let result = find_devspin_yml_parallel(nonexistent.to_str().unwrap());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("does not exist"));
    }

    #[test]
    fn test_handles_root_is_file() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("example.txt");
        File::create(&file_path).unwrap();
        let result = find_devspin_yml_parallel(file_path.to_str().unwrap());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("is not a directory"));
    }

    #[test]
    fn test_root_optimization_skips_full_search() {
        // File in root should be returned immediately without timeout risks
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("devspin.yml");
        File::create(&config_path).unwrap();

        // Use short timeout to verify it doesn't search with timeout
        let result = find_devspin_yml_with_timeout(
            temp_dir.path().to_str().unwrap(),
            Duration::from_nanos(1),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), config_path);
    }
}
