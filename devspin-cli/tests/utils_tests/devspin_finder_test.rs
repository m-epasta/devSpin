use devspin_cli::utils::devspin_finder::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_finds_file_in_root() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("devspin.yml");
        File::create(&config_path).unwrap();
        
        let result = find_devspin_yml_parallel(temp_dir.path().to_str().unwrap());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), config_path.to_string_lossy());
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
    }

    #[test]
    fn test_skips_node_modules() {
        let temp_dir = tempdir().unwrap();
        let node_modules = temp_dir.path().join("node_modules");
        fs::create_dir(&node_modules).unwrap();
        let config_path = node_modules.join("devspin.yml");
        File::create(&config_path).unwrap(); // This should be skipped
        
        let result = find_devspin_yml_parallel(temp_dir.path().to_str().unwrap());
        assert!(result.is_err()); // Should NOT find it in node_modules
    }

    #[test]
    fn test_handles_missing_root() {
        let result = find_devspin_yml_parallel("/nonexistent/path/123456");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("not found") || err.contains("does not exist"));
    }
}