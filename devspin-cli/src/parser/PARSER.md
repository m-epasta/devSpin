# Parser Documentation

Simple guide for the parser module.

---

## `devspin_generator.rs`

**Purpose:** Parse and serialize DevSpin configuration files.

---

## Data Structures

| Struct | Description |
|--------|-------------|
| `DevSpinConfigHeader` | Header with `version`, `name`, `base` |
| `SpinFeature` | Feature config with `enabled` and `modes` |
| `Service` | Docker-like service (`image`, `ports`, `environment`) |
| `Task` | Named task with a `command` |
| `DevSpinConfig` | Full config (header + content) |

---

## Usage

### 1. Parse a config file

```rust
use crate::parser::devspin_generator::{DevSpinGenerator, DEFAULT_DEVSPIN_CONFIG};

// Parse from string array
let config = DevSpinGenerator::generate(DEFAULT_DEVSPIN_CONFIG)?;

println!("Project: {}", config.header.name);
println!("Base: {}", config.header.base);
```

### 2. Create a config manually

```rust
use crate::parser::devspin_generator::*;
use std::collections::HashMap;

let header = DevSpinConfigHeader::new("1.0", "my-project", "ubuntu:latest");

let content = DevSpinConfigContent {
    spin_features: HashMap::from([
        ("lint".to_string(), SpinFeature { enabled: true, modes: vec!["all".to_string()] })
    ]),
    system: vec![],
    env: HashMap::new(),
    services: vec![],
    tasks: vec![
        Task { name: "build".to_string(), command: "cargo build".to_string() }
    ],
};

let config = DevSpinConfig::new(header, content);
```

### 3. Write config to string

```rust
let yaml_string = config.write();
println!("{}", yaml_string);
```

**Output:**
```yaml
version: 1.0
name: my-project
base: ubuntu:latest

spin-features:
  lint:
    enabled: true
    modes: ["all"]

tasks:
  - name: build
    command: cargo build
```

---

## Functions

| Function | Description |
|----------|-------------|
| `DevSpinGenerator::generate(input)` | Parse `&[&str]` → `DevSpinConfig` |
| `DevSpinConfig::write(&self)` | Serialize config → YAML string |
| `DevSpinConfigHeader::new(v, n, b)` | Create header with version, name, base |

---

## Config File Format

```yaml
# Header (required)
version: 1.0
name: my-project
base: ubuntu:latest

# Spin Features (optional)
spin-features:
  lint:
    enabled: true
    modes: [all]

# System packages (optional)
system:
  - curl
  - git

# Environment variables (optional)
env:
  NODE_ENV: production

# Services (optional)
services:
  - name: postgres
    image: postgres:15
    ports:
      - "5432:5432"
    environment:
      POSTGRES_PASSWORD: secret

# Tasks (optional)
tasks:
  - name: build
    command: cargo build
  - name: test
    command: cargo test
```

---

## Quick Import

```rust
use crate::parser::devspin_generator::{
    DevSpinGenerator,
    DevSpinConfig,
    DevSpinConfigHeader,
    DevSpinConfigContent,
    SpinFeature,
    Service,
    Task,
    DEFAULT_DEVSPIN_CONFIG,
};
```
