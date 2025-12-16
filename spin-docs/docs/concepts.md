---
title: Core Concepts
description: Understanding SPN targets, modes, and builtins
sidebar_position: 3
---

import Diagram from '@site/src/components/Diagram';

# 🎯 Core Concepts

spin is built around a few key concepts that make "declare once, run anywhere" possible. Understanding these will help you write better spin files and troubleshoot issues.

##  The Mental Model

Think of spin as a **universal translator** for application manifests:

1. **You describe** what your app needs (language, dependencies, ports, etc.)
2. **spin understands** your intent and generates platform-specific configs
3. **Multiple platforms** get the configs they need (Docker, Kubernetes, AWS, etc.)

No more writing the same thing in 5 different formats!

## Targets

Targets are the **output platforms** spin can generate configs for. Each target speaks a different "language":

### Built-in Targets

| Target | Purpose | Generates | When to Use |
|--------|---------|-----------|-------------|
| `docker` | Containerization | `Dockerfile`, `docker-compose.yml` | Local development, container deployment |
| `kubernetes` | Orchestration | `deployment.yaml`, `service.yaml`, `configmap.yaml` | Production Kubernetes clusters |
| `terraform` | Infrastructure | `main.tf`, `variables.tf` | Cloud infrastructure (AWS, GCP, Azure) |
| `compose` | Local orchestration | `docker-compose.yml` | Multi-service local development |
| `helm` | K8s packaging | `Chart.yaml`, `values.yaml` | Kubernetes application packaging |

### Target Syntax

```spn
; Or target multiple platforms
run docker, kubernetes {
  ; Shared configuration
} TODO: translate this block into real spin

cfg
{
  name: "my-app",
  version: 1.0.0,
}

@service>web |
@runtime>node |
@interpret>build {
  port: 3000,
  main: "./server/server.js",
}

@container>docker |
@transpile>root ; here, the file is also transpiled to build/ but if you want, you can add a path to have a copy (copy will not be used).
{
  ; Docker specific code
}
```

### How Targets Work

<Diagram type="targets" />

1. **SPN Parser** reads your manifest
2. **Target Processor** converts to platform-specific format
3. **Platform Tools** execute the generated configs

## 🎛️ Modes

Modes control **how** SPN executes your application. Different modes for different situations:

### Execution Modes

| Mode | Behavior | Use Case |
|------|----------|----------|
| `execute-required` | Must execute, fail if runtime missing | Production deployments |
| `execute-if-available` | Try to execute, fallback to transpile | Development (default) |
| `transpile-only` | Generate configs only, don't run | CI/CD pipelines |

### Mode Syntax

```spn
#! spn 1.0

; Development mode (default)
run local {
  mode: execute-if-available  ; Try to run, generate if can't
}

; Production mode
run production {
  mode: execute-required      ; Must run, fail if dependencies missing
  scaling: { min: 3, max: 10 }
}

; CI mode
run ci {
  mode: transpile-only        ; Just generate configs
}
```

### Mode Detection

SPN automatically detects your environment:

- **Local development**: Uses `execute-if-available`
- **CI/CD**: Uses `transpile-only`
- **Production**: Uses `execute-required`

## 🔧 Builtins

Builtins are SPN's **built-in functions** that handle common tasks. They're prefixed with `#!` and use the file's version:

### Common Builtins

| Builtin | Purpose | Example |
|---------|---------|---------|
| `validate-config` | Check configuration | `#! validate-config` |
| `optimize-image` | Optimize container images | `#! optimize-image` |
| `health-check` | Add health checks | `#! health-check` |
| `exec-scripts` | Execute shell scripts | `#! exec-scripts` |

### Builtin Syntax

```spn
#! spn 1.0  ; Version applies to all builtins

app "web" {
  type: "node"
  #! validate-config  ; Uses v1.0
}

scripts {
  #! exec-scripts {   ; Uses v1.0
    #!/usr/bin/env bash
    echo "Building app..."
    npm run build
  }
}
```

### Version Caching

The `#! spn X.Y` at the top **sets the version for the entire file**. All builtins use this version automatically.

## 🏷️ Types

Types tell SPN what kind of application you're building:

### Application Types

| Type | Description | Examples |
|------|-------------|----------|
| `node` | Node.js application | Express, Next.js, Nuxt |
| `python` | Python application | Flask, Django, FastAPI |
| `go` | Go application | Gin, Echo, net/http |
| `rust` | Rust application | Actix, Rocket, Warp |
| `java` | Java application | Spring Boot, Micronaut |
| `dotnet` | .NET application | ASP.NET Core |
| `static` | Static website | HTML/CSS/JS |

### Type Detection

SPN auto-detects types from your files:

```spn
; Auto-detected from package.json
app "api" {
  type: "node"  ; Detected automatically
}

; Explicit type override
app "worker" {
  type: "python:3.11"  ; Specific version
}
```

## 📦 Dependencies

Dependencies are services your app needs (databases, caches, etc.):

### Built-in Dependencies

```spn
app "web" {
  type: "node"
  needs: [postgres, redis, rabbitmq]
}
```

SPN automatically:
- Generates connection configs
- Sets environment variables
- Creates network connections
- Handles service discovery

### Custom Dependencies

```spn
app "web" {
  needs: [
    postgres {
      version: "15"
      port: 5432
    },
    redis {
      version: "7"
      port: 6379
    }
  ]
}
```

## 🔄 Workspaces

Workspaces define your **project structure**:

```spn
{
  name: "my-fullstack-app"
  version: "1.0.0"
  workspace: "./app"  ; Base directory
}

app "frontend" {
  type: "react"
  workspace: "./frontend"  ; Relative to base
}

app "backend" {
  type: "node"
  workspace: "./backend"   ; Relative to base
}
```

## 🚀 Execution Flow

Here's what happens when you run `spin run`:

1. **Parse** SPN file
2. **Validate** configuration
3. **Detect** environment and available tools
4. **Generate** target-specific configs
5. **Execute** (if mode allows)
6. **Monitor** and provide feedback

## 🎨 Configuration Blocks

Configuration blocks customize SPN's behavior:

```spn
@cfg {
  #features: [
    runtime,        ; Enable runtime execution
    transpile,      ; Enable config generation
    scripts,        ; Enable script execution
    health-check    ; Enable health monitoring
  ]

  #dependencies: [
    config<json>: "package.json"     ; JSON dependency
    config<yaml>: "docker-compose.yml" ; YAML dependency
  ]
}
```

## 📚 Next Steps

Now that you understand the core concepts:

- **[First Project](/docs/first-project)** - Apply concepts to a real app
- **[FAQ](/docs/faq)** - Common questions and troubleshooting

## 🆘 Need Help?

- **[Community](https://discord.gg/spin)** - Get help from others

---

**Got it?** [Try migrating your first app →](/docs/first-project)
