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
| `container` | Containerization | `Dockerfile`, `podman` | Local development, container deployment |
| `cluster` | Orchestration | `deployment.yaml`, `service.yaml`, `configmap.yaml` | Production Kubernetes clusters |
| `terraform` | Infrastructure | `main.tf`, `variables.tf` | Cloud infrastructure (AWS, GCP, Azure) |
| `compose` | Local orchestration | `docker-compose.yml` | Multi-service local development |
| `helm` | K8s packaging | `Chart.yaml`, `values.yaml` | Kubernetes application packaging |

### Target Syntax

```spn

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

@cluster>kubernetes |
@transpile>build ; does not make a copy in root
{
  ; kubernetes specific code
}

run>cfg {

}

```

### How Targets Work

<Diagram type="targets" />

1. **spin Parser** reads your manifest
2. **Target Processor** converts to platform-specific format
3. **Platform Tools** execute the generated configs

## 🎛️ Modes

Modes control **how** spin executes your application. Different modes for different situations:
**NOTE**: modes are defined as macros version `@[]`

### Execution Modes

| Mode | Behavior | Use Case |
|------|----------|----------|
| `exec-required` | Must execute, fail if runtime missing | Production deployments |
| `try-exec` | Try to execute, fallback to transpile | Development (default) |
| `transpile-only` | Generate configs only, don't run | CI/CD pipelines |

### Mode Syntax

```spn
#! spn 1.0

; Development mode (default)
@[try-exec]
run local {
  ; Try to run, generate if can't
}

; Production mode
@[exec-required]
run production {      
  ; Must run, fail if dependencies missing
  scaling: { min: 3, max: 10 }
}

; CI mode
@[transpile-only]
run ci {
  ; Just generate configs
}
```

### Mode Detection

spin automatically detects your environment:

- **Local development**: Uses `try-exec`
- **CI/CD**: Uses `transpile-only`
- **Production**: Uses `execute-required`

## 🔧 Builtins

Builtins are spin's **built-in functions** that handle common tasks. They're prefixed with `#!` and use the file's version:

### Common Builtins

| Builtin | Purpose | Example |
|---------|---------|---------|
| `validate-config` | Check configuration | `#! validate-config` |
| `optimize-image` | Optimize container images | `#! optimize-image` |
| `health-check` | Add health checks | `#! health-check` |
| `exec-scripts` | Make shell scripts executable | `#! exec-scripts` |

### Builtin Syntax

```spn
#! spn 1.0  ; Version applies to all builtins

@service>web |
@runtime>node |
@interpret
{
  ; node config - You can point to package.json or to his dep (if defined) as #cfg<name/of/your/file>
}

scripts { ; keyword
  #! exec-scripts {   ; Uses v1.0
    #!/usr/bin/env bash
    echo "Building app..."
    npm run build
  }
}
```

### Version Caching

The `#! spn X.Y` at the top **sets the version for the entire file**. All builtins use this version automatically.


## Dependencies

Dependencies are services your app needs (databases, caches, etc.):

### Built-in Dependencies

```spn
@service>web |
@runtime>node |
@intepret
{
  #deps: [postgres, redis, rabbitmq]
  ; OR
  #deps: #cfg>package.json ; # means define (used for configs)
}

```

Spin let you define deps or to point to a config file

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

@service>web |
@runtime>node |
@intepret
{
  #deps: [
    postgres {
      version: "15",
      port: 5432,
    },
    redis { version: 7, port: 6379, }
  ]
}
```

## Workspaces

Workspaces define your **project structure**:

```spn
{
  name: "my-fullstack-app"
  version: "1.0.0"
  workspace: "./app"  ; Base directory
}

@service>web |
@runtime>js-fw | ; fw stand for framework 
@fw>react |
@intepret
{
 TODO: complete
}
```

## 🚀 Execution Flow

Here's what happens when you run `spin run`:

1. **Parse** spin file
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
