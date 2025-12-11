use devspin_cli::utils::create_devspin_file::*;

mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_file_creation_success() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let temp_path = temp_dir
            .path()
            .to_str()
            .expect("Failed to convert temp dir path to string");

        let created_path = create_cfg_file(temp_path)?;
        let expected_path = temp_dir.path().join("devspin.yml");

        assert_eq!(created_path, expected_path);
        assert!(created_path.exists(), "File should exist");
        assert!(created_path.is_file(), "Should be a file, not a directory");

        Ok(())
    }

    #[test]
    fn test_file_creation_in_subdirectory() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let subdir = temp_dir.path().join("config");

        fs::create_dir(&subdir)?;

        let subdir_path = subdir
            .to_str()
            .ok_or("Failed to convert subdirectory path to string")?;

        let created_path = create_cfg_file(subdir_path)?;

        assert_eq!(created_path, subdir.join("devspin.yml"));
        assert!(created_path.exists());

        Ok(())
    }

    #[test]
    #[should_panic(expected = "CreateFileFailed")]
    fn test_file_creation_fails_invalid_path() {
        let _result = create_cfg_file("/point/to/the/butt/of/caroline").expect("CreateFileFailed");
    }

    #[test]
    fn test_returns_correct_pathbuf() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let temp_path = temp_dir
            .path()
            .to_str()
            .expect("Failed to convert path to string");

        let result = create_cfg_file(temp_path);

        assert!(result.is_ok());
        let path_buf = result.expect("Should be able to get the value");

        assert_eq!(path_buf.file_name().unwrap(), "devspin.yml");
        assert!(path_buf.has_root() || !path_buf.is_absolute());

        Ok(())
    }

    #[test]
    fn test_creates_empty_file() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path().to_str().unwrap();

        let created_path = create_cfg_file(temp_path)?;

        let metadata = fs::metadata(&created_path)?;
        assert_eq!(metadata.len(), 0); // Empty file

        Ok(())
    }
}
