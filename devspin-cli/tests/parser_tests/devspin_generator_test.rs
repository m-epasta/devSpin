mod tests {
    use devspin_cli::parser::devspin_generator::{DevSpinGenerator, DEFAULT_DEVSPIN_CONFIG};

    #[test]
    fn test_generate_default_config() {
        let config = DevSpinGenerator::generate(DEFAULT_DEVSPIN_CONFIG)
            .expect("Failed to generate default config");

        assert_eq!(config.header.version, "1.0");
        assert_eq!(config.header.name, "my-devspin-project");
        assert_eq!(config.header.base, "ubuntu:latest");

        // Default config has tasks but no system/env in the constant definition currently used in tests
        // Let's verify what we have.
        assert!(!config.content.tasks.is_empty());
        assert_eq!(config.content.tasks[0].name, "build");
    }

    #[test]
    fn test_parse_full_config() {
        let input = &[
            "version: 1.0",
            "name: test-project",
            "base: rust:1.75",
            "",
            "system:",
            "  - git",
            "  - curl",
            "",
            "env:",
            "  PORT: 8080",
            "  RUST_LOG: debug",
            "",
            "tasks:",
            "  - name: test",
            "    command: cargo test",
        ];

        let config = DevSpinGenerator::generate(input).expect("Failed to parse full config");

        // Header
        assert_eq!(config.header.version, "1.0");
        assert_eq!(config.header.name, "test-project");
        assert_eq!(config.header.base, "rust:1.75");

        // System
        assert_eq!(config.content.system.len(), 2);
        assert!(config.content.system.contains(&"git".to_string()));
        assert!(config.content.system.contains(&"curl".to_string()));

        // Env
        assert_eq!(config.content.env.get("PORT"), Some(&"8080".to_string()));
        assert_eq!(
            config.content.env.get("RUST_LOG"),
            Some(&"debug".to_string())
        );

        // Tasks
        assert_eq!(config.content.tasks.len(), 1);
        assert_eq!(config.content.tasks[0].name, "test");
        assert_eq!(config.content.tasks[0].command, "cargo test");
    }

    #[test]
    fn test_missing_header_fields() {
        let input = &[
            "version: 1.0",
            // Missing name and base
        ];

        let result = DevSpinGenerator::generate(input);
        assert!(result.is_err());
    }

}
