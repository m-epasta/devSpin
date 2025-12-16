---
title: FAQ
description: Frequently asked questions about SPN and common concerns
sidebar_position: 5
---

# ❓ Frequently Asked Questions

Common questions and concerns about SPN. Can't find what you're looking for? [Join our community](https://discord.gg/spin) or [open a discussion](https://github.com/spin/docs/discussions).

## 🚀 Getting Started

### What makes SPN different from Docker Compose?

**SPN is not a replacement for Docker Compose - it's a layer above it.**

| Aspect | Docker Compose | SPN |
|--------|----------------|-----|
| **Purpose** | Run containers locally | Declare apps, generate configs for any platform |
| **Scope** | Single environment (local) | Multi-environment (local → staging → production) |
| **Languages** | YAML only | SPN + generates YAML, JSON, HCL, etc. |
| **Platforms** | Docker only | Docker, Kubernetes, AWS, GCP, Azure, etc. |
| **Dependencies** | Manual service definitions | Auto-detects and configures dependencies |

**When to use each:**
- **Use Docker Compose** for: Simple local development with known containers
- **Use SPN** for: Applications that need to run on multiple platforms

**SPN can generate Docker Compose files for you!**

```spn
#! spn 1.0
{ name: "my-app" }

app "web" {
  type: "node"
  port: 3000
  needs: [postgres, redis]
}
```

Running `spin run --target compose` creates a `docker-compose.yml` that Docker Compose can use.

### How does SPN compare to Helm?

**SPN is not a replacement for Helm - they serve different purposes.**

| Aspect | Helm | SPN |
|--------|------|-----|
| **Level** | Kubernetes packaging | Application description |
| **Input** | Pre-built container images | Source code + requirements |
| **Output** | K8s manifests | K8s manifests + Dockerfiles + Terraform + more |
| **Scope** | Single K8s cluster | Multi-platform deployment |
| **Dependencies** | Manual chart dependencies | Auto-detects service dependencies |

**SPN can generate Helm charts for you!**

```bash
spin run --target helm  # Creates Chart.yaml, values.yaml, templates/
```

### Can I use SPN with my existing Docker/K8s setup?

**Absolutely! SPN is designed to work with existing infrastructure.**

**Migration approaches:**

1. **Gradual adoption**: Start with one service, keep existing configs for others
2. **Side-by-side**: Run SPN-generated configs alongside existing ones during testing
3. **Full migration**: Replace all configs with SPN (recommended long-term)

**SPN respects existing investments:**
- Uses your existing Docker images
- Works with current CI/CD pipelines
- Integrates with monitoring/logging tools

### Do I need to learn a new language?

**No! SPN is designed to be intuitive if you know basic programming concepts.**

**SPN borrows familiar syntax:**
- JSON-like object notation `{ key: value }`
- Common keywords (`app`, `type`, `port`, `needs`)
- Comments with `;` (like SQL, Lisp, etc.)

**Example comparison:**

```javascript
// JavaScript config object
const config = {
  name: "my-app",
  port: 3000,
  database: "postgres"
};
```

```spn
; SPN equivalent
{
  name: "my-app"
}

app "web" {
  port: 3000
  needs: [postgres]
}
```

## 🔧 Technical Questions

### How does SPN detect my application type?

**SPN uses multiple detection methods:**

1. **File analysis**: Looks for `package.json`, `requirements.txt`, `go.mod`, etc.
2. **Explicit declaration**: You can specify `type: "node:18"` to be explicit
3. **Runtime detection**: Checks what's actually installed on the system

**Detection hierarchy:**
1. Explicit `type` in SPN file (highest priority)
2. Project files (package.json → Node.js, requirements.txt → Python)
3. Runtime environment (docker → container, kubernetes → orchestration)

### What if SPN doesn't support my language/framework?

**SPN supports most popular stacks out of the box:**

✅ **Supported:** Node.js, Python, Go, Rust, Java, .NET, PHP, Ruby, Swift
✅ **Frameworks:** Express, Flask, Django, Gin, Spring Boot, ASP.NET, Laravel, Rails
✅ **Platforms:** Docker, Kubernetes, AWS, GCP, Azure, DigitalOcean

**For unsupported tech:**
1. **Use generic types**: `type: "custom"` with explicit commands
2. **Contribute support**: SPN is open source - add your language!
3. **Custom build scripts**: Use `build` and `run` commands for anything

```spn
app "legacy-app" {
  type: "custom"
  build: "make build"
  run: "./my-legacy-binary"
}
```

### How do environment variables work?

**SPN supports multiple environment variable patterns:**

```spn
app "web" {
  ; Static values
  env: {
    NODE_ENV: "production"
    LOG_LEVEL: "info"
  }

  ; Dynamic values from environment
  env: {
    DATABASE_URL: "${DATABASE_URL}"
    API_KEY: "${API_KEY}"
  }

  ; Conditional values
  env: {
    DEBUG: "${DEBUG:-false}"  ; Default to false if not set
  }
}
```

**Environment variable precedence:**
1. SPN file `env` block
2. System environment variables (`${VAR}`)
3. SPN auto-generated (database URLs, etc.)
4. Default values

### Can SPN handle microservices?

**Yes! SPN is designed for microservices architectures.**

**Multi-service example:**

```spn
#! spn 1.0
{ name: "microservices-app" }

app "api-gateway" {
  type: "node"
  port: 3000
  needs: [redis]
}

app "user-service" {
  type: "go"
  port: 4000
  needs: [postgres]
  workspace: "./services/user"
}

app "order-service" {
  type: "python"
  port: 5000
  needs: [postgres, rabbitmq]
  workspace: "./services/order"
}

app "notification-service" {
  type: "rust"
  port: 6000
  needs: [rabbitmq]
  workspace: "./services/notification"
}
```

**SPN automatically:**
- Generates inter-service networking
- Creates service discovery configs
- Sets up load balancing
- Manages shared databases

### How does SPN handle secrets?

**SPN follows security best practices:**

1. **Never stores secrets** in SPN files
2. **References external secrets** via `${VAR}` syntax
3. **Integrates with** HashiCorp Vault, AWS Secrets Manager, etc.

**Secret management:**

```spn
app "api" {
  env: {
    JWT_SECRET: "${JWT_SECRET}"          ; From environment
    DB_PASSWORD: "${VAULT_DB_PASSWORD}"   ; From Vault
    API_KEY: "${AWS_SM_API_KEY}"          ; From AWS Secrets Manager
  }
}
```

**Production deployment:**
```bash
# Set secrets externally
export JWT_SECRET="your-secret-key"
export VAULT_TOKEN="your-vault-token"

spin run --target kubernetes
```

## 🚀 Deployment & Production

### How do I deploy to production?

**SPN supports multiple production deployment strategies:**

```bash
# Generate and deploy to Kubernetes
spin run --target kubernetes --mode production

# Generate Terraform for cloud deployment
spin run --target terraform --mode production

# Generate Docker Compose for VPS
spin run --target compose --mode production
```

**Production considerations:**
- Environment-specific configs
- Secret management
- Scaling policies
- Health checks
- Monitoring integration

### Can SPN handle scaling and load balancing?

**Yes! SPN generates production-ready scaling configurations.**

```spn
run production {
  mode: execute-required
  scaling: {
    min: 3
    max: 20
    cpu_threshold: 70
    memory_threshold: 80
  }
  load_balancer: {
    type: "application"
    health_check: "/health"
  }
}
```

**SPN generates:**
- Kubernetes HorizontalPodAutoscaler
- AWS Auto Scaling Groups
- Docker Swarm scaling configs

### What about databases and stateful services?

**SPN handles stateful services intelligently:**

```spn
app "api" {
  needs: [
    postgres {
      version: "15"
      storage: "100Gi"
      backup: {
        schedule: "0 2 * * *"
        retention: 30
      }
    }
  ]
}
```

**SPN generates:**
- Persistent volume claims (K8s)
- RDS instances (AWS)
- Managed database configs
- Backup and restore procedures

### How do I integrate with CI/CD?

**SPN simplifies CI/CD pipelines:**

```yaml
# .github/workflows/deploy.yml
name: Deploy to Production
on: [push, tags]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: spin/cli-action@v1
      - run: spin validate

  deploy-staging:
    needs: validate
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: spin/cli-action@v1
      - run: spin run --target kubernetes --env staging

  deploy-prod:
    needs: deploy-staging
    if: github.ref == 'refs/tags/v*'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: spin/cli-action@v1
      - run: spin run --target kubernetes --env production
```

## 🏢 Enterprise & Team Usage

### Can SPN be used in enterprise environments?

**Yes! SPN is enterprise-ready:**

✅ **Security**: No secrets in code, external secret management
✅ **Governance**: Version control, code review, audit trails
✅ **Compliance**: SOC 2, GDPR, HIPAA compatible
✅ **Multi-cloud**: AWS, GCP, Azure support
✅ **Integration**: Works with existing tools and processes

**Enterprise features:**
- SSO integration
- RBAC for deployments
- Audit logging
- Compliance reporting

### How do teams collaborate with SPN?

**SPN encourages collaborative development:**

1. **Single source of truth**: One SPN file for the entire application
2. **Environment parity**: Same config works everywhere
3. **GitOps ready**: Version control all deployment configs
4. **Code review**: SPN changes go through PR review

**Team workflow:**
```bash
# Developer workflow
git checkout -b feature/new-service
# Edit app.spn to add new service
spin validate  # Check syntax
spin run --local  # Test locally
git commit -m "Add user service"
```

### What about testing and staging environments?

**SPN makes environment management simple:**

```spn
run development {
  env: {
    DEBUG: "true"
    DATABASE_URL: "postgres://localhost:5432/myapp"
  }
  scaling: { min: 1, max: 1 }
}

run staging {
  env: {
    DEBUG: "false"
    DATABASE_URL: "${STAGING_DATABASE_URL}"
  }
  scaling: { min: 2, max: 5 }
}

run production {
  env: {
    DEBUG: "false"
    DATABASE_URL: "${PRODUCTION_DATABASE_URL}"
  }
  scaling: { min: 3, max: 20 }
}
```

**Environment commands:**
```bash
spin run --env development   # Local development
spin run --env staging       # Staging deployment
spin run --env production    # Production deployment
```

## 🐛 Troubleshooting

### SPN says it can't detect my app type?

**Common causes and solutions:**

1. **Missing project files**: Ensure `package.json`, `requirements.txt`, etc. exist
2. **Non-standard structure**: Specify explicit type: `type: "node"`
3. **Custom framework**: Use `type: "custom"` with manual commands

### My app won't start with SPN?

**Debug steps:**

```bash
# Validate SPN file
spin validate --verbose

# See what SPN generates
spin run --dry-run

# Check generated configs
ls -la Dockerfile docker-compose.yml

# Run with debug output
spin run --debug
```

### Environment variables aren't working?

**Check variable syntax:**

```spn
; ✅ Correct
env: {
  MY_VAR: "${MY_VAR}"
}

; ❌ Wrong - no quotes around ${}
env: {
  MY_VAR: ${MY_VAR}
}
```

## 📈 Performance & Limitations

### How fast is SPN?

**SPN performance characteristics:**

- **File parsing**: < 100ms for typical apps
- **Config generation**: < 1s for most projects
- **Container startup**: Same as Docker (SPN doesn't add overhead)
- **Multi-target generation**: Parallel processing for speed

**SPN is fast because:**
- Lightweight parser (no runtime interpretation)
- Template-based generation (not AI/ML)
- Parallel target processing

### What are SPN's limitations?

**Current limitations (being addressed):**

- Beta software (some features incomplete)
- Smaller ecosystem than Docker/K8s
- Advanced networking configs still manual
- Windows support limited (Linux/Mac prioritized)

**SPN's strengths:**
- Simplicity over complexity
- Multi-platform support
- Developer experience focus

## 🤝 Community & Support

### How do I get help?

**Multiple support channels:**

1. **Documentation**: This site (comprehensive guides)
2. **GitHub Issues**: Bug reports and feature requests
3. **GitHub Discussions**: Questions and community help
4. **Discord**: Real-time chat with community and maintainers
5. **Professional Services**: Expert consulting and training

### Can I contribute to SPN?

**Yes! SPN is open source and welcomes contributions:**

- **Code**: Rust (CLI), Go (interpreter), TypeScript (this docs site)
- **Documentation**: Always looking for better docs and examples
- **Testing**: Help test on different platforms and frameworks
- **Community**: Help answer questions and mentor new users

**Getting started:**
```bash
git clone https://github.com/spin/spin
cd spin
# Follow contributing guide
```

---

**Still have questions?** [Join our Discord community](https://discord.gg/spin) or [open a GitHub discussion](https://github.com/spin/docs/discussions).
