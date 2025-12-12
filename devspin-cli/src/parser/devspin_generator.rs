use crate::error::DevSpinError;
use std::collections::HashMap;

/// Default configuration template used for initialization.
pub const DEFAULT_DEVSPIN_CONFIG: &[&str] = &[
    "version: 1.0",
    "name: my-devspin-project",
    "base: ubuntu:latest",
    "",
    "spin-features:",
    "  lint:",
    "    enabled: true",
    "    modes: [all]",
    "",
    "# Add your tasks below",
    "tasks:",
    "  - name: build",
    "    command: cargo build",
];

#[derive(Debug, Clone, PartialEq)]
pub struct DevSpinConfigHeader {
    pub version: String,
    pub name: String,
    pub base: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpinFeature {
    pub enabled: bool,
    pub modes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Service {
    pub name: String,
    pub image: String,
    pub ports: Vec<String>,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DevSpinConfigContent {
    pub spin_features: HashMap<String, SpinFeature>,
    pub system: Vec<String>,
    pub env: HashMap<String, String>,
    pub services: Vec<Service>,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DevSpinConfig {
    pub header: DevSpinConfigHeader,
    pub content: DevSpinConfigContent,
}

impl DevSpinConfigHeader {
    pub fn new(
        version: impl Into<String>,
        name: impl Into<String>,
        base: impl Into<String>,
    ) -> Self {
        Self {
            version: version.into(),
            name: name.into(),
            base: base.into(),
        }
    }
}

impl DevSpinConfig {
    pub fn new(header: DevSpinConfigHeader, content: DevSpinConfigContent) -> Self {
        Self { header, content }
    }

    pub fn write(&self) -> String {
        let mut output = format!(
            "version: {}\nname: {}\nbase: {}\n\n",
            self.header.version, self.header.name, self.header.base
        );

        if !self.content.spin_features.is_empty() {
            output.push_str("spin-features:\n");
            for (name, feature) in &self.content.spin_features {
                output.push_str(&format!("  {}:\n", name));
                output.push_str(&format!("    enabled: {}\n", feature.enabled));
                output.push_str(&format!("    modes: {:?}\n", feature.modes));
            }
            output.push('\n');
        }

        if !self.content.system.is_empty() {
            output.push_str("system:\n");
            for sys in &self.content.system {
                output.push_str(&format!("  - {}\n", sys));
            }
            output.push('\n');
        }

        if !self.content.env.is_empty() {
            output.push_str("env:\n");
            for (key, val) in &self.content.env {
                output.push_str(&format!("  {}: {}\n", key, val));
            }
            output.push('\n');
        }

        if !self.content.services.is_empty() {
            output.push_str("services:\n");
            for service in &self.content.services {
                output.push_str(&format!("  - name: {}\n", service.name));
                output.push_str(&format!("    image: {}\n", service.image));
                if !service.ports.is_empty() {
                    output.push_str("    ports:\n");
                    for port in &service.ports {
                        output.push_str(&format!("      - \"{}\"\n", port));
                    }
                }
                if !service.environment.is_empty() {
                    output.push_str("    environment:\n");
                    for (k, v) in &service.environment {
                        output.push_str(&format!("      {}: {}\n", k, v));
                    }
                }
            }
            output.push('\n');
        }

        if !self.content.tasks.is_empty() {
            output.push_str("tasks:\n");
            for task in &self.content.tasks {
                output.push_str(&format!(
                    "  - name: {}\n    command: {}\n",
                    task.name, task.command
                ));
            }
        }

        output
    }
}

pub struct DevSpinGenerator;

impl DevSpinGenerator {
    /// Generates the configuration object from an array of strings (template).
    pub fn generate(input: &[&str]) -> Result<DevSpinConfig, DevSpinError> {
        let mut header_lines = Vec::new();
        let mut content_lines = Vec::new();
        let mut in_header = true;
        let mut version_found = false;
        let mut name_found = false;
        let mut base_found = false;

        for &line in input {
            if in_header {
                if line.trim().is_empty() && version_found && name_found && base_found {
                    in_header = false;
                    continue;
                }

                if line.trim().starts_with("version:") {
                    version_found = true;
                    header_lines.push(line);
                } else if line.trim().starts_with("name:") {
                    name_found = true;
                    header_lines.push(line);
                } else if line.trim().starts_with("base:") {
                    base_found = true;
                    header_lines.push(line);
                } else if !version_found && !name_found && !base_found && line.trim().is_empty() {
                    // skip initial empty lines
                } else if version_found && name_found && base_found {
                    // If we have all fields and hit something else, switch to content
                    in_header = false;
                    content_lines.push(line);
                } else {
                    // Still in header potential area
                    header_lines.push(line);
                }
            } else {
                content_lines.push(line);
            }
        }

        let header = Self::parse_header(&header_lines)?;
        let content = Self::parse_content(&content_lines)?;

        Ok(DevSpinConfig { header, content })
    }

    fn parse_header(lines: &[&str]) -> Result<DevSpinConfigHeader, DevSpinError> {
        let mut version = None;
        let mut name = None;
        let mut base = None;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some(v) = trimmed.strip_prefix("version:") {
                version = Some(v.trim().to_string());
            } else if let Some(n) = trimmed.strip_prefix("name:") {
                name = Some(n.trim().to_string());
            } else if let Some(b) = trimmed.strip_prefix("base:") {
                base = Some(b.trim().to_string());
            }
        }

        Ok(DevSpinConfigHeader {
            version: version.ok_or(DevSpinError::MissingField("version".to_string()))?,
            name: name.ok_or(DevSpinError::MissingField("name".to_string()))?,
            base: base.ok_or(DevSpinError::MissingField("base".to_string()))?,
        })
    }

    fn parse_content(lines: &[&str]) -> Result<DevSpinConfigContent, DevSpinError> {
        let mut spin_features = HashMap::new();
        let mut system = Vec::new();
        let mut env = HashMap::new();
        let mut services = Vec::new();
        let mut tasks = Vec::new();

        let mut current_section = "";

        // State for nested parsing
        let mut current_feature_name = None;
        let mut current_service: Option<Service> = None;
        let mut current_task_name = None;

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed == "spin-features:" {
                current_section = "spin-features";
                continue;
            } else if trimmed == "system:" {
                current_section = "system";
                continue;
            } else if trimmed == "env:" {
                current_section = "env";
                continue;
            } else if trimmed == "services:" {
                current_section = "services";
                continue;
            } else if trimmed == "tasks:" {
                current_section = "tasks";
                continue;
            }

            match current_section {
                "spin-features" => {
                    if trimmed.ends_with(':') {
                        // New feature block
                        let feature_name = trimmed.trim_end_matches(':');
                        current_feature_name = Some(feature_name.to_string());
                        spin_features.insert(
                            feature_name.to_string(),
                            SpinFeature {
                                enabled: false,
                                modes: vec![],
                            },
                        );
                    } else if let Some(name) = &current_feature_name {
                        if let Some(val) = trimmed.strip_prefix("enabled:") {
                            if let Some(feature) = spin_features.get_mut(name) {
                                feature.enabled = val.trim() == "true";
                            }
                        } else if let Some(val) = trimmed.strip_prefix("modes:") {
                            if let Some(feature) = spin_features.get_mut(name) {
                                let modes_str =
                                    val.trim().trim_start_matches('[').trim_end_matches(']');
                                feature.modes =
                                    modes_str.split(',').map(|s| s.trim().to_string()).collect();
                            }
                        }
                    }
                }
                "system" => {
                    if let Some(sys) = trimmed.strip_prefix("- ") {
                        system.push(sys.trim().to_string());
                    }
                }
                "env" => {
                    if let Some((key, val)) = trimmed.split_once(':') {
                        env.insert(key.trim().to_string(), val.trim().to_string());
                    }
                }
                "services" => {
                    if let Some(name) = trimmed.strip_prefix("- name:") {
                        // Push previous service if exists
                        if let Some(svc) = current_service.take() {
                            services.push(svc);
                        }
                        current_service = Some(Service {
                            name: name.trim().to_string(),
                            image: String::new(),
                            ports: Vec::new(),
                            environment: HashMap::new(),
                        });
                    } else if let Some(svc) = &mut current_service {
                        if let Some(img) = trimmed.strip_prefix("image:") {
                            svc.image = img.trim().to_string();
                        } else if trimmed == "ports:" {
                            // Prepare for ports
                        } else if trimmed.starts_with("- \"") && trimmed.contains(':') {
                            // Port mapping like - "8080:8080"
                            let port = trimmed.trim_start_matches("- \"").trim_end_matches('"');
                            svc.ports.push(port.to_string());
                        } else if trimmed == "environment:" {
                            // Prepare for env
                        } else if let Some((k, v)) = trimmed.split_once(':') {
                            // Environment var for service
                            // Note: This simple parser might confuse this with top-level keys if indentation isn't tracked.
                            // For MVP, we assume correct structure.
                            svc.environment
                                .insert(k.trim().to_string(), v.trim().to_string());
                        }
                    }
                }
                "tasks" => {
                    if let Some(name) = trimmed.strip_prefix("- name:") {
                        current_task_name = Some(name.trim().to_string());
                    } else if let Some(cmd) = trimmed.strip_prefix("command:") {
                        if let Some(name) = current_task_name.take() {
                            tasks.push(Task {
                                name,
                                command: cmd.trim().to_string(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        // Push last service
        if let Some(svc) = current_service {
            services.push(svc);
        }

        Ok(DevSpinConfigContent {
            spin_features,
            system,
            env,
            services,
            tasks,
        })
    }
}
