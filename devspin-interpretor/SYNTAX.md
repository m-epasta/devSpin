# SPN (Spin) Language Specification
## Version 1.0 Alpha

## 📜 Overview

SPN (Spin) is a declarative application manifest language that enables "declare once, deploy everywhere." You describe your application's needs once, and SPN adapts to generate or execute configurations for any environment.

## 🚀 Quick Start

```spn
#! spn 1.0

{
  name: my-webapp
  version: 1.0.0
  workspace: "./app"
}

@cfg {
  #features:
   - runtime
   - transpile:
    file
   - scripts:
    exec-scripts
}

@[reuse(container as main-container)]
@main-container>docker |
@runtime>spn:actual {
  #transpile>file {
    FROM node:18-alpine
    WORKDIR /app
    COPY . .
    RUN npm install
    CMD ["npm", "start"]
  }
}

scripts {
  #! exec-scripts {
    #!/usr/bin/env bash
    echo "Building with SPN v1.0"
    docker build -t my-webapp .
  }
}
```

## 📖 Syntax Guide

### 1. File Structure

Every SPN file must begin with a version declaration:

```spn
#! spn 1.0
```

**Important**: This version is **cached** for the entire file. All `#!` builtins will use this version.

### 2. Comments

```spn
; This is a single-line comment
#! spn 1.0 ; Comments can follow statements

; Multi-line comments aren't natively supported
; Use multiple single-line comments
```

### 3. Imports and Aliases

```spn
/! v as actual           ; Import 'v' as alias 'actual'
/! stdlib as spn-std     ; Import standard library
```

### 4. Configuration Blocks

#### 4.1 Application Metadata
```spn
{
  name: my-application
  version: 1.0.0
  description: "My awesome app"
  workspace: "./src"     ; Base directory for relative paths
}
```

#### 4.2 Feature Configuration
```spn
@cfg {
  #features:
   - runtime
   - transpile:
    file
   - scripts:
    exec-scripts
  
  #dependencies:
    - config<json>: "package.json"
    - config<yaml>: "docker-compose.yml"
  
  #vc:                    ; Version Control
    - config<.gitignore>: ".gitignore"
    - config<gitattributes>: ".gitattributes"
}
```

### 5. Type-Parameterized References

SPN uses angle brackets for type constraints on file references:

```spn
config<json>: "package.json"                    ; JSON file
config<yaml>: "docker-compose.yml"              ; YAML file
main<ext(md)>: "README.md"                      ; Markdown extension
main<name("CHANGELOG", ext(".md"))>: "CHANGELOG.md" ; Named with extension
doc<schema("api.schema.json")>: "api-docs.json" ; With schema validation
```

### 6. Target Definitions

#### 6.1 Basic Syntax
```spn
@target-type>name |          ; Pipe indicates multiple targets
@another-target>name {       ; Start of target block
  ; Content specific to this target
  ; (e.g., Dockerfile, K8s YAML, etc.)
}
```

#### 6.2 Target Configuration (Parentheses)
```spn
@container>myapp (
  transpile: "Dockerfile"
  execute: "docker run $image"
  requires: ["docker"]
) {
  ; Target content goes here
}
```

#### 6.3 Reusable Templates
```spn
@[reuse(container as app-container)]  ; Create reusable template
@[reuse(deployment as k8s-deployment)]

@app-container>docker { ... }         ; Use the template
@k8s-deployment>kubernetes { ... }    ; Use another template
```

### 7. Builtins with Version Checking

All `#!` builtins automatically use the file's cached version:

```spn
#! spn 1.0                    ; Sets version for entire file

@target>docker {
  #! validate-image           ; Uses v1.0
  #! optimize-layers         ; Uses v1.0
}

scripts {
  #! exec-scripts {           ; Uses v1.0
    #!/usr/bin/env bash       ; Bash shebang (different!)
    #! internal-check         ; Would be Bash syntax error!
  }
}
```

### 8. Script Blocks

```spn
scripts {
  #! exec-scripts {           ; SPN v1.0 builtin
    #!/usr/bin/env bash       ; Script language shebang
    echo "Hello from bash"
    docker build -t $IMAGE .
  }
  
  #! exec-scripts {
    #!/usr/bin/env python3
    print("Hello from Python")
  }
}
```

### 9. Pipe Operator for Multiple Targets

```spn
@frontend>docker |           ; First target
@backend>docker |            ; Second target  
@database>postgres {         ; Third target
  ; Shared content or configuration
  ; All targets receive this context
}
```

## 🎯 Execution Modes

### Mode Declaration
```spn
#! spn 1.0
@mode>transpile-only         ; Only generate config files

@mode>execute-if-available   ; Try to execute, fallback to transpile (DEFAULT)

@mode>execute-required       ; Must execute, fail if runtime missing
```

## 🔤 Character Reference

| Character | Purpose | Example |
|-----------|---------|---------|
| `#!` | File version declaration | `#! spn 1.0` |
| `#!` | Builtin call (uses cached version) | `#! exec-scripts` |
| `#!/` | Embedded language shebang | `#!/bin/bash` |
| `/!` | Import/alias | `/! v as actual` |
| `;` | Comment | `; This is a comment` |
| `#` | Section header | `#features:` |
| `@` | Decorator/annotation | `@cfg` |
| `@[` | Macro/decorator with arguments | `@[reuse(...)]` |
| `<>` | Type parameters | `config<json>` |
| `\|` | Target chain operator | `@target1 \| @target2` |
| `:` | Key-value separator | `name: value` |
| `-` | List item | `- runtime` |
| `{}` | Block delimiters | `{ content }` |
| `()` | Configuration parameters | `(transpile: ".Dockerfile")` |

## 📝 Complete Example

```spn
#! spn 1.0
/! stdlib as spn

{
  name: fullstack-app
  version: 2.1.0
  workspace: "."
}

@cfg {
  #features:
   - runtime
   - transpile:
    file
   - scripts:
    exec-scripts
    health-check
  
  #dependencies:
    - config<json>: "package.json"
    - config<json>: "package-lock.json"
    - config<yaml>: "docker-compose.yml"
  
  #vc:
    - config<.gitignore>: ".gitignore"
}

@[reuse(web-container as app-frontend)]
@[reuse(api-container as app-backend)]
@[reuse(db-container as app-database)]

@mode>execute-if-available

@app-frontend>docker (
  transpile: "Dockerfile.frontend"
  execute: "docker-compose up frontend"
) {
  FROM node:18-alpine
  WORKDIR /app
  COPY frontend/package*.json ./
  RUN npm ci
  COPY frontend/ .
  RUN npm run build
  EXPOSE 3000
  CMD ["npm", "start"]
}

@app-backend>docker |
@app-database>postgres {
  #! validate-configuration
  environment:
    NODE_ENV: production
    DB_HOST: postgres
    DB_PORT: 5432
}

scripts {
  #! exec-scripts {
    #!/usr/bin/env bash
    set -e
    
    echo "🚀 Starting full deployment"
    echo "SPN Version: 1.0"
    
    # Build and deploy
    docker-compose build
    docker-compose up -d
    
    # Health check
    sleep 10
    curl -f http://localhost:3000/health || exit 1
    
    echo "✅ Deployment successful!"
  }
  
  #! health-check {
    #!/usr/bin/env bash
    curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/health
  }
}
```

## ⚠️ Rules and Constraints

1. **One version per file**: The `#! spn X.Y` must be the first non-comment line
2. **Version caching**: All `#!` builtins use the file's declared version
3. **No mid-file version switching**: Unlike some languages, you cannot change SPN versions within a file
4. **Target content is opaque**: The SPN parser doesn't interpret target content (Dockerfiles, YAML, etc.)
5. **Embedded shebangs**: `#!/bin/bash` etc. are only interpreted by their respective language processors
6. **Pipe operator**: Must be followed by another target definition on the next line

## 🔍 Parser Behavior

The SPN interpreter works in two phases:

1. **SPN Parsing Phase**: Only interprets SPN-specific constructs (`#!`, `@`, `#`, `/!`, etc.)
2. **Target Dispatch Phase**: Delegates target content to specialized processors (Docker, Kubernetes, etc.)

This means:
- Large Dockerfiles/K8s YAML don't slow down SPN parsing
- Each target can be processed in parallel
- New target types can be added without modifying the core parser

## 🆚 Comparison to Other Formats

| Feature | SPN | Docker Compose | Kubernetes YAML | Helm |
|---------|-----|----------------|-----------------|------|
| Multi-platform | ✅ | ❌ (Docker only) | ❌ (K8s only) | ❌ (K8s only) |
| Version management | ✅ | ❌ | ❌ | ✅ |
| Builtin scripts | ✅ | ❌ | ❌ | ❌ |
| Type-safe config references | ✅ | ❌ | ❌ | ❌ |
| Parallel execution | ✅ | ❌ | ❌ | ❌ |

## 📚 Next Steps

1. Install the SPN interpreter
2. Create your first `.spn` file
3. Run `spn validate` to check syntax
4. Execute with `spn run --target docker`

---

*SPN Syntax v1.0 - Subject to change during alpha development*