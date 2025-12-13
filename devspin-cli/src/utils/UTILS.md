# Utils Documentation

Simple guide for each utility in this module.

---

## 1. `create_devspin_file.rs`

**Purpose:** Create an empty `devspin.yml` config file.

### Usage

```rust
use crate::utils::create_devspin_file::create_cfg_file;

// Create devspin.yml in a directory
let path = create_cfg_file("/path/to/project")?;
println!("Created: {}", path.display());
```

### Function

| Function | Input | Output |
|----------|-------|--------|
| `create_cfg_file(root: &str)` | Directory path | `Result<PathBuf, ProcessError>` |

---

## 2. `devspin_finder.rs`

**Purpose:** Find `devspin.yml` in a directory tree using parallel search.

### Usage

```rust
use crate::utils::devspin_finder::{find_devspin_yml_parallel, find_devspin_yml_with_timeout};
use std::time::Duration;

// Quick search (30s timeout)
let path = find_devspin_yml_parallel("/path/to/search")?;

// Custom timeout
let path = find_devspin_yml_with_timeout("/path/to/search", Duration::from_secs(10))?;
```

### Functions

| Function | Description |
|----------|-------------|
| `find_devspin_yml_parallel(root)` | Search with default 30s timeout |
| `find_devspin_yml_with_timeout(root, timeout)` | Search with custom timeout |

### Features
- ✅ Parallel search using Rayon
- ✅ Skips common junk directories (`node_modules`, `target`, `.git`, etc.)
- ✅ Accepts: `devspin.yml`, `devspin.yaml`, `.devspin.yml`
- ✅ Checks root directory first (fast path)

---

## 3. `root_finder.rs`

**Purpose:** Find the project root by scanning upward for common indicators.

### Usage

```rust
use crate::utils::root_finder::{get_root, get_root_no_param};
use std::path::PathBuf;

// From specific directory
let root = get_root(PathBuf::from("/home/user/project/src/lib"))?;

// From current working directory
let root = get_root_no_param()?;
```

### Functions

| Function | Description |
|----------|-------------|
| `get_root(current_dir)` | Find root from given path |
| `get_root_no_param()` | Find root from current working dir |

### Priority Order (searches groups in order)

1. **Version Control** (highest): `.git`, `.svn`, `.hg`, `.bzr`
2. **Package Managers**: `Cargo.toml`, `package.json`, `go.mod`, etc.
3. **IDE Configs**: `.vscode`, `.idea`
4. **CI/CD**: `.github`, `.circleci`
5. **Build Tools**: `Makefile`, `Dockerfile`, etc.
6. **Project Dirs** (lowest): `src`, `lib`, `tests`, etc.

### Limits
- Max depth: 10 levels up
- Skips system paths (`/`, `/tmp`, `/usr`, `C:/Windows`, etc.)

---

## Quick Import

```rust
// In your code
use crate::utils::{
    create_devspin_file::create_cfg_file,
    devspin_finder::find_devspin_yml_parallel,
    root_finder::get_root_no_param,
};
```
