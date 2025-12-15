# **SPN - Spin**
## **The Multi-Target Application Manifest Language**

---

## **📋 Quick Start**

```yaml
#! spn v1.0

@target>docker {
  from: "node:18"
  copy: [".", "/app"]
  cmd: "node index.js"
}

@target>kubernetes {
  deployment:
    replicas: 3
    image: "myapp:latest"
}
```

```bash
# Generate Dockerfile
$ spn generate --target docker

# Generate K8s manifests  
$ spn generate --target kubernetes

# Execute deployment (if runtimes available)
$ spn run --target kubernetes
```

---

## **🎯 What Problem Does SPN Solve?**

### **The Multi-Platform Configuration Nightmare**
Modern applications require configuration in multiple formats:
- `Dockerfile` for containers
- `k8s/*.yaml` for Kubernetes
- `.github/workflows/*.yml` for CI/CD
- `docker-compose.yml` for local development
- Cloud provider configs (AWS, GCP, Azure)
- Infrastructure as Code (Terraform, Pulumi)

**All describing the SAME application in DIFFERENT languages.**

---

## **✨ SPN's Vision: Declare Once, Deploy Everywhere**

```
        [app.spn] ← Your declaration
           /  |  \
          /   |   \
    [Docker] [K8s] [Cloud]
        \     |     /
         \    |    /
      [Your Running Application]
```

You declare **WHAT** you want, SPN figures out **HOW** to make it happen based on available runtimes.

---

## **🔧 Execution Modes**

SPN adapts to your environment with three execution strategies:

```yaml
#! spn v1.0

# Mode 1: Always generate config files
@mode>transpile-only

# Mode 2: Execute if runtime available, else generate files (DEFAULT)
@mode>execute-if-available

# Mode 3: Must execute, fail if runtime missing
@mode>execute-required
```

### **Example: Smart Database Setup**
```yaml
#! spn v1.0
@mode>execute-if-available

@target>database {
  postgres:
    version: 15
    storage: 100GB
    backups: true
}
```

**What SPN does:**
- If Docker available → Runs PostgreSQL container
- If Podman available → Runs Podman container  
- If cloud credentials available → Creates cloud database
- If none available → Generates configuration files

---

## **🚀 Key Use Cases**

### **1. Local Development**
```yaml
#! spn v1.0
@mode>execute-if-available

@target>dev {
  frontend:
    framework: react
    port: 3000
    hot-reload: true
    
  backend:
    runtime: node
    port: 8080
    watch: true
    
  database:
    postgres:
      version: 15
      
  cache:
    redis: {}
}
```
One command: `spn run --target dev`

### **2. Production Deployment**
```yaml
#! spn v1.0
@mode>execute-required  # Must have production tools

@target>production {
  deployment:
    replicas: 3
    auto-scale:
      min: 3
      max: 10
      
  database:
    postgres:
      high-availability: true
      read-replicas: 2
      
  monitoring:
    prometheus: true
    grafana: true
    alerts: true
}
```

### **3. Multi-Cloud Strategy**
```yaml
#! spn v1.0
@mode>execute-if-available

@target>aws {
  # Generates/executes: ECS, RDS, ElastiCache, etc.
}

@target>gcp {
  # Generates/executes: Cloud Run, Cloud SQL, Memorystore, etc.
}

@target>azure {
  # Generates/executes: AKS, Azure SQL, Redis, etc.
}
```

---

## **🎨 Language Philosophy**

### **Declarative, Not Imperative**
```yaml
# RIGHT: Declare what you want
@target>app {
  api:
    port: 8080
    health-check: /health
}

# WRONG: Don't specify how to do it
# @target>app {
#   setup: () => { installDocker(); buildImage(); }  # NO!
# }
```

### **Convention Over Configuration**
```yaml
# SPN knows common patterns
@target>node-app {
  # Auto-inferred: npm install, node_modules volume, PORT env
}

@target>python-app {
  # Auto-inferred: pip install, virtualenv, requirements.txt
}

@target>rails-app {
  # Auto-inferred: bundle install, database.yml, assets
}
```

### **Progressive Disclosure**
```yaml
# Beginner (just run my app)
@target>simple {
  from: "python:3.11"
  cmd: "python app.py"
}

# Intermediate (add services)
@target>with-services {
  database: postgres
  cache: redis
  queue: rabbitmq
}

# Expert (full production)
@target>production-ready {
  ingress:
    domains: ["app.example.com"]
    tls: letsencrypt
    
  security:
    waf: true
    ddos-protection: true
    
  backup:
    schedule: "0 2 * * *"
    retention: 30d
}
```

---

## **🔍 How It Works**

### **1. Parse & Validate**
```bash
$ spn validate app.spn
✓ Syntax valid
✓ Targets: docker, kubernetes, terraform
✓ Requirements: docker (optional), kubectl (optional)
```

### **2. Runtime Detection**
```bash
$ spn check --target kubernetes

Runtime check for 'kubernetes':
✓ kubectl installed (v1.28.0)
✓ Kubernetes context set (prod-cluster)
✗ Helm installed (optional)
→ Mode: execute-if-available → Will execute deployment
```

### **3. Execution Strategy**
```yaml
# Based on @mode and available runtimes:
execute-if-available → Has kubectl? → kubectl apply
                   ↘ No kubectl? → Generate YAML files

transpile-only → Always generate files
execute-required → Must have runtime, else error
```

---

## **🎯 For Whom?**

### **Individual Developers**
- **"I just want to run my app"**
- `spn init` → Choose stack → Running in 60 seconds
- No need to learn Docker, Kubernetes, cloud configs

### **Startups & Small Teams**
- **"We need consistency across environments"**
- Single `.spn` file for dev, staging, production
- New hires productive on day one

### **Platform Engineers**
- **"We need standardization"**
- Create `@template>company-standard` for your org
- Enforce security, monitoring, compliance
- Extend SPN with custom targets

### **Open Source Maintainers**
- **"Make contribution easy"**
- Contributors don't need Docker/K8s/cloud expertise
- One `.spn` file instead of 10 config files

---

## **📈 Evolution Path**

### **Phase 1: Core Targets**
```yaml
# MVP: Most common needs
@target>docker {}      # Dockerfile + docker-compose
@target>kubernetes {}  # K8s manifests
@target>ci-cd {}       # GitHub Actions, GitLab CI
```

### **Phase 2: Cloud Providers**
```yaml
# Major cloud platforms
@target>aws {}    # ECS, App Runner, EKS
@target>gcp {}    # Cloud Run, GKE
@target>azure {}  # Container Apps, AKS
```

### **Phase 3: Ecosystem**
```yaml
# Plugin system
@plugin>pulumi {}     # Generate Pulumi code
@plugin>cdk {}        # Generate AWS CDK
@plugin>ansible {}    # Generate Ansible playbooks
@plugin>nomad {}      # Generate Nomad jobs
```

---

## **🤔 Why SPN, Not X?**

| Tool | Purpose | SPN Difference |
|------|---------|----------------|
| **Docker Compose** | Local multi-container | Also generates production configs |
| **Helm** | K8s package manager | Not K8s-specific, simpler syntax |
| **Pulumi/CDK** | Infrastructure as Code | Focus on apps, not infrastructure |
| **Batect/Task** | Task runners | Also generates deployment configs |
| **Nix** | Reproducible builds | Simpler, container-focused |

**SPN's niche**: The **application runtime layer** - your app's needs, not the underlying infrastructure.

---

## **🎮 The "Spin" Metaphor**

```yaml
# Traditional: Manual translation
[Dockerfile] → [K8s YAML] → [Cloud config] → [Helm Chart]
   ↳ You maintain 4 files that drift apart

# SPN: Automatic adaptation
[app.spn] → @spin>docker → Dockerfile
         → @spin>k8s    → deployment.yaml
         → @spin>cloud  → terraform/
         
# You "spin" one declaration to multiple outputs
```

---

## **🚀 Getting Started**

```bash
# Install
curl -fsSL https://spn.dev/install.sh | sh

# Initialize project
spn init

# Choose your stack
? Select application type:
  ○ Node.js API
  ○ Python Django  
  ○ React frontend
  ○ Full-stack app
  ○ Custom

# Generated .spn file
cat app.spn

# Run it!
spn run --target dev

# Deploy to production
spn run --target production
```

---

## **📚 Real-World Analogy**

**SPN is to deployment what...**
- **HTML is to documents** (declarative, not imperative)
- **SQL is to databases** (say what you want, not how)
- **Docker is to servers** (abstraction over complexity)

**It's a portable application manifest** that tells tools how to run your application, without locking you into any specific tool.

---

## **🎯 TL;DR**

**SPN is:** A declarative language that describes your application's runtime needs once, then adapts to execute or generate configurations for any environment.

**SPN is NOT:** Yet another infrastructure tool, programming language, or YAML generator.

**The promise:** Stop writing the same application configuration in Docker, Kubernetes, cloud, and CI/CD formats. Declare it once in SPN, then **spin it anywhere**.

---

*"One declaration to run them all, one declaration to find them, one declaration to bring them all, and in the cloud to bind them."*

---

## **🔮 Future Vision**

Imagine:
- Frameworks shipping `.spn` files instead of Dockerfiles
- Cloud providers offering `@target>their-platform` plugins  
- Teams sharing `@template>` configurations internally
- SPN becoming the `.gitignore` or `package.json` of deployment

**The goal:** Make deploying applications as easy as `git push` once was.

---

Ready to spin?