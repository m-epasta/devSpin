---
title: Welcome to SPIN
description: Declare once. Run anywhere. The multi-target application manifest language.
slug: /
sidebar_position: 1
---

import {Redirect} from '@docusaurus/router';

<Redirect to="/docs/quickstart" />

# Welcome to SPIN

**Declare once. Run anywhere.** Stop writing the same configurations in different languages. Write your app manifest once, and let SPIN generate Dockerfiles, Kubernetes YAML, Terraform configs, and more.

## ⚡ Quick Example

```spn
#! spn 1.0
{ name: "my-webapp" }

app "web" {
  type: "node"
  port: 3000
  needs: [postgres, redis]
}
```

**One file → Multiple platforms:**

- **Docker**: Generates `Dockerfile` + `docker-compose.yml`
- **Kubernetes**: Creates deployment, service, and configmap YAMLs
- **AWS**: Produces Terraform configs for ECS + RDS + ElastiCache
- **Local**: Runs your app with hot-reload development

## 🎯 What SPIN Does

| Problem | Traditional Approach | SPIN Solution |
|---------|---------------------|---------------|
| **Multi-platform deployment** | Write separate configs for Docker, K8s, cloud | One SPN file works everywhere |
| **Service orchestration** | Manual docker-compose + K8s manifests | Auto-detects and configures dependencies |
| **Environment management** | Different configs per environment | Smart defaults + environment overrides |
| **Local development** | Complex setup scripts | `spin run --local` just works |

## 🚀 Get Started in 60 Seconds

Ready to try SPIN? It takes less than a minute to get your first app running:

1. **Install SPIN CLI**
   ```bash
   curl -fsSL https://spin.dev/install.sh | sh
   ```

2. **Create your app manifest**
   ```bash
   spin init my-app
   cd my-app
   ```

3. **Run it**
   ```bash
   spin run
   ```

That's it! SPIN detects your stack, sets up databases, and starts your app.

## 📚 Learn More

- **[Quick Start Guide](/docs/quickstart)** - Get running in 5 minutes
- **[Core Concepts](/docs/concepts)** - Understand targets, modes, and builtins
- **[First Project](/docs/first-project)** - Migrate an existing app
- **[Examples](/examples)** - Real-world SPN files for common stacks

## 🎉 Why Developers Love SPIN

> "SPIN reduced our deployment complexity by 80%. One manifest file instead of 15 different configs." - Senior DevOps Engineer

> "Finally, a tool that understands my app needs. No more wrestling with YAML." - Full Stack Developer

## 🤝 Join the Community

SPIN is open source and community-driven. Join thousands of developers simplifying their deployments:

- 🐙 **[GitHub](https://github.com/spin)**
- 💬 **[Discord Community](https://discord.gg/spin)**
- 🐦 **[Twitter @spin_dev](https://twitter.com/spin_dev)**

---

**Ready to declare once and run anywhere?** [Start with the Quick Start Guide →](/docs/quickstart)
