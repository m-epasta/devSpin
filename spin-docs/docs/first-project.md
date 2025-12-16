---
title: First Real Project
description: Step-by-step guide to migrating an existing app to SPN
sidebar_position: 4
---

import Steps from '@site/src/components/Steps';

# 🚀 Your First Real Project

Ready to migrate an existing application to SPN? This guide walks you through converting common application stacks step-by-step. By the end, you'll have a working SPN manifest that replaces multiple config files.

## 🎯 What You'll Learn

- Analyze existing application structure
- Convert Docker/K8s configs to SPN
- Handle complex dependencies
- Test the migration
- Deploy to multiple platforms

## 📋 Prerequisites

- Existing application (Node.js, Python, Go, etc.)
- Basic understanding of [core concepts](/docs/concepts)
- SPN CLI installed ([Quick Start](/docs/quickstart))

## Step 1: Analyze Your Current Setup

<Steps>

### 1.1 Examine Your Application Structure

First, understand what you're working with:

```bash
# List your project files
find . -name "*.json" -o -name "*.yml" -o -name "*.yaml" -o -name "Dockerfile*" | head -20

# Check package managers
ls -la package.json requirements.txt go.mod pom.xml *.csproj 2>/dev/null || true

# Look for existing configs
ls -la docker-compose.yml Dockerfile kubernetes/ terraform/ .github/workflows/
```

**What to look for:**
- Main application files (`server.js`, `app.py`, `main.go`, etc.)
- Configuration files (`config.json`, `.env`, etc.)
- Docker/K8s manifests
- CI/CD pipelines
- Database migrations
- Build scripts

### 1.2 Identify Key Components

Map your application components:

| Component | Current Config | SPN Equivalent |
|-----------|----------------|----------------|
| Web server | `app.listen(3000)` | `port: 3000` |
| Database | `DATABASE_URL=postgres://...` | `needs: [postgres]` |
| Cache | `REDIS_URL=redis://...` | `needs: [redis]` |
| Environment | `.env` file | `env: { NODE_ENV: "production" }` |

</Steps>

## Step 2: Choose Your Migration Path

<Steps>

### Option A: Node.js Express API (Most Common)

**Current setup typically has:**
- `package.json` with dependencies
- `server.js` or `index.js`
- `Dockerfile` and `docker-compose.yml`
- `.env` for configuration

**SPN equivalent:**

```spn
#! spn 1.0
{ name: "my-api" }

app "api" {
  type: "node"
  port: 3000
  needs: [postgres, redis]
  env: {
    NODE_ENV: "production"
    JWT_SECRET: "${JWT_SECRET}"
  }
}
```

### Option B: React + Node.js Fullstack

**Current setup:**
- `frontend/` with React app
- `backend/` with Express API
- Multiple Dockerfiles
- `docker-compose.yml` for orchestration

**SPN equivalent:**

```spn
#! spn 1.0
{ name: "fullstack-app" }

app "frontend" {
  type: "react"
  port: 3000
  workspace: "./frontend"
}

app "backend" {
  type: "node"
  port: 4000
  workspace: "./backend"
  needs: [postgres]
}
```

### Option C: Python Flask/Django API

**Current setup:**
- `requirements.txt` or `Pipfile`
- `app.py` or Django settings
- `Dockerfile`
- Database configuration

**SPN equivalent:**

```spn
#! spn 1.0
{ name: "flask-api" }

app "api" {
  type: "python"
  port: 8000
  needs: [postgres]
  env: {
    FLASK_ENV: "production"
    SECRET_KEY: "${SECRET_KEY}"
  }
}
```

### Option D: Go API Service

**Current setup:**
- `go.mod` and Go source files
- `Dockerfile`
- Environment variables

**SPN equivalent:**

```spn
#! spn 1.0
{ name: "go-api" }

app "api" {
  type: "go"
  port: 8080
  needs: [postgres]
}
```

</Steps>

## Step 3: Create Your SPN Manifest

<Steps>

### 3.1 Start with Basic Structure

Create `app.spn` in your project root:

```spn
#! spn 1.0

{
  name: "my-app"
  version: "1.0.0"
  description: "My application migrated to SPN"
}

; Add your applications here
```

### 3.2 Add Application Definitions

Based on your analysis, add app blocks:

```spn
app "web" {
  type: "node"        ; or "python", "go", etc.
  port: 3000          ; main port
  run: "npm start"    ; start command (optional, auto-detected)
}
```

### 3.3 Configure Dependencies

Add any services your app needs:

```spn
app "web" {
  type: "node"
  port: 3000
  needs: [postgres, redis]  ; databases, caches, message queues
}
```

### 3.4 Set Environment Variables

Migrate from `.env` or config files:

```spn
app "web" {
  env: {
    NODE_ENV: "production"
    API_KEY: "${API_KEY}"        ; Use ${} for secrets
    DATABASE_URL: "postgres://db:5432/myapp"
  }
}
```

### 3.5 Handle Special Cases

**Multiple apps in one project:**

```spn
app "frontend" {
  type: "react"
  port: 3000
  workspace: "./client"
}

app "backend" {
  type: "node"
  port: 4000
  workspace: "./server"
  needs: [postgres]
}
```

**Custom build steps:**

```spn
app "web" {
  type: "node"
  build: "npm run build"    ; custom build command
  run: "npm start"          ; custom run command
}
```

**Health checks:**

```spn
app "api" {
  type: "go"
  health: "/health"         ; health check endpoint
}
```

</Steps>

## Step 4: Test Your Migration

<Steps>

### 4.1 Validate SPN File

```bash
spin validate
```

This checks your SPN syntax and catches errors early.

### 4.2 Run Locally

```bash
spin run --local
```

SPN will:
- Detect your application type
- Generate Docker configs if needed
- Start your app and dependencies
- Provide access URLs

### 4.3 Compare with Original

**Before migration:**
```bash
docker-compose up -d
# Manual port forwarding, environment setup
```

**After migration:**
```bash
spin run
# Everything automated
```

### 4.4 Test Different Targets

```bash
# Generate Kubernetes manifests
spin run --target kubernetes --dry-run

# Generate Terraform configs
spin run --target terraform --dry-run

# Just validate without running
spin validate
```

</Steps>

## Step 5: Handle Advanced Scenarios

<Steps>

### 5.1 Database Migrations

If you have database migrations:

```spn
scripts {
  #! exec-scripts {
    #!/usr/bin/env bash
    echo "Running database migrations..."
    npx prisma migrate deploy
  }
}
```

### 5.2 Custom Docker Configuration

For advanced Docker needs:

```spn
run docker {
  base_image: "node:18-alpine"
  expose_ports: [3000, 4000]
  volumes: ["/app/data:/data"]
}
```

### 5.3 Environment-Specific Configs

Different settings per environment:

```spn
run development {
  env: {
    DEBUG: "true"
    DATABASE_URL: "postgres://localhost:5432/myapp"
  }
}

run production {
  env: {
    DEBUG: "false"
    DATABASE_URL: "${DATABASE_URL}"
  }
  scaling: { min: 2, max: 10 }
}
```

### 5.4 CI/CD Integration

Replace complex CI configs with simple SPN commands:

```yaml
# .github/workflows/deploy.yml
name: Deploy
on: [push]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: spin/cli-action@v1
      - run: spin run --target kubernetes
```

</Steps>

## Step 6: Deploy to Production

<Steps>

### 6.1 Choose Your Platform

```bash
# Docker deployment
spin run --target docker --mode production

# Kubernetes deployment
spin run --target kubernetes --mode production

# Cloud deployment (generates Terraform)
spin run --target terraform --mode production
```

### 6.2 Environment Variables

Set production secrets:

```bash
export DATABASE_URL="postgres://prod-db:5432/myapp"
export API_KEY="your-production-api-key"
spin run --target kubernetes
```

### 6.3 Scaling and Monitoring

```spn
run production {
  mode: execute-required
  scaling: {
    min: 3
    max: 20
    cpu_threshold: 70
  }
  health: {
    path: "/health"
    interval: 30
    timeout: 10
  }
}
```

</Steps>

## 🐛 Troubleshooting Migration Issues

### Common Problems & Solutions

**"SPN can't detect my app type"**
```spn
; Explicit type declaration
app "web" {
  type: "node:18"  ; Be specific
}
```

**"Dependencies not connecting"**
```spn
; Check service names match
needs: [postgres, redis]  ; Use standard names

; Or specify custom config
needs: [
  postgres { port: 5432, database: "myapp" }
]
```

**"Environment variables not working"**
```spn
env: {
  MY_VAR: "${MY_VAR}"  ; Use ${} for external vars
  STATIC_VAR: "value"  ; Direct values work too
}
```

**"Build failing"**
```bash
spin validate --verbose  ; Detailed error info
spin run --dry-run       ; See what would be generated
```

## 📊 Migration Checklist

- [ ] Analyzed current application structure
- [ ] Identified all components and dependencies
- [ ] Created basic SPN manifest
- [ ] Added application definitions
- [ ] Configured dependencies
- [ ] Set environment variables
- [ ] Tested locally with `spin run`
- [ ] Validated with `spin validate`
- [ ] Tested different targets
- [ ] Handled special cases (migrations, custom builds)
- [ ] Deployed to staging/production
- [ ] Updated CI/CD pipelines
- [ ] Removed old config files (optional)

## 🎉 Success Metrics

**Before Migration:**
- 5+ config files (Dockerfile, docker-compose.yml, k8s YAMLs, etc.)
- Manual environment setup
- Platform-specific knowledge required
- Error-prone deployments

**After Migration:**
- 1 SPN file
- Automatic environment detection
- Multi-platform support
- Consistent deployments

## 📚 Next Steps

- **[FAQ](/docs/faq)** - Common questions and troubleshooting

## 🆘 Need Help?

- **[Community](https://discord.gg/spin)** - Get help from others
