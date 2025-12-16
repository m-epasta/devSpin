---
title: Quick Start
description: Get SPN running in 5 minutes
slug: /quickstart
sidebar_position: 2
---

# 🚀 Quick Start

Get SPN running in your environment in less than 5 minutes. By the end of this guide, you'll have a working application that SPN manages completely.

## 📋 Prerequisites

- **Node.js 18+** (for the example app)
- **Terminal/Command Prompt**
- **Internet connection** (to download SPN)

No Docker, Kubernetes, or cloud accounts required for this quick start!

## 1. Install SPN CLI

Choose your installation method:

### Option A: One-liner (Recommended)

```bash
curl -fsSL https://spin.dev/install.sh | sh
```

This downloads and installs the SPN CLI to `~/.spin/bin/spin`.

### Option B: Manual Download

1. Go to [GitHub Releases](https://github.com/spin/cli/releases)
2. Download the binary for your platform
3. Add to your PATH

### Option C: Build from Source

```bash
git clone https://github.com/spin/cli.git
cd cli
cargo build --release
# Add target/release/spin to your PATH
```

### Verify Installation

```bash
spin --version
# Should output: spin 1.0.0
```

## 2. Create Your First SPN Project

SPN can initialize a project for you:

```bash
# Create a new directory
mkdir my-first-spin-app
cd my-first-spin-app

# Initialize with SPN
spin init
```

This creates:
- `app.spn` - Your application manifest
- Basic project structure

### Manual Creation (Alternative)

If you prefer to create files manually:

```bash
mkdir my-first-spin-app
cd my-first-spin-app
```

Create `app.spn`:

```spn
#! spn 1.0
{ name: "hello-spin" }

app "web" {
  type: "node"
  port: 3000
  run: "npm start"
}
```

Create a simple `package.json`:

```json
{
  "name": "hello-spin",
  "version": "1.0.0",
  "scripts": {
    "start": "node server.js"
  },
  "dependencies": {
    "express": "^4.18.2"
  }
}
```

Create `server.js`:

```javascript
const express = require('express');
const app = express();
const port = process.env.PORT || 3000;

app.get('/', (req, res) => {
  res.send(`
    <!DOCTYPE html>
    <html>
      <head><title>Hello SPIN!</title></head>
      <body style="font-family: Arial, sans-serif; text-align: center; padding: 50px;">
        <h1>🚀 Hello from SPIN!</h1>
        <p>Your app is running successfully.</p>
        <p><strong>Time:</strong> ${new Date().toLocaleString()}</p>
        <p><strong>Environment:</strong> ${process.env.NODE_ENV || 'development'}</p>
      </body>
    </html>
  `);
});

app.listen(port, () => {
  console.log(`Server running at http://localhost:${port}`);
});
```

Install dependencies:

```bash
npm install
```

## 3. Run Your App

Now for the magic! Run your app with SPN:

```bash
spin run
```

**What happens next:**

1. **SPN analyzes** your `app.spn` file
2. **Detects** you have a Node.js app
3. **Generates** a `Dockerfile` (if Docker is available)
4. **Creates** `docker-compose.yml` for orchestration
5. **Starts** your app with hot-reload

You should see output like:

```
✅ SPN detected Node.js project
📦 Generated Dockerfile
🐳 Generated docker-compose.yml
🚀 Starting application...
   Web server: http://localhost:3000
   Dev mode: enabled (hot-reload active)
```

## 4. Verify It's Working

Open your browser to **http://localhost:3000**

You should see a page that says "Hello from SPIN!" with the current time.

### Test Hot-Reload

Modify `server.js` - change the greeting text. Save the file. The page should automatically refresh with your changes!

## 5. Explore What SPN Created

SPN generates platform-specific configs for you. Check what was created:

```bash
ls -la
```

You might see:
- `Dockerfile` - Container definition
- `docker-compose.yml` - Multi-service orchestration
- `.spin/` - SPN working directory

### Try Different Targets

SPN can target different platforms. Try:

```bash
# Generate Kubernetes manifests
spin run --target kubernetes

# Generate Terraform for AWS
spin run --target terraform

# Just validate without running
spin validate
```

## 🎉 Congratulations!

You've successfully:
- ✅ Installed SPN CLI
- ✅ Created an application manifest
- ✅ Run your first SPN-managed app
- ✅ Seen hot-reload in action

## 🚀 Next Steps

Now that you have SPN working, explore:

- **[Core Concepts](/docs/concepts)** - Learn about targets, modes, and builtins
- **[First Real Project](/docs/first-project)** - Migrate an existing application
- **[FAQ](/docs/faq)** - Common questions and troubleshooting

## 🆘 Having Issues?

- Check `spin --help` for command options
- Run `spin validate` to check your SPN file
- Join our [Discord Community](https://discord.gg/spin) for help

**Need to stop the app?** Press `Ctrl+C` in your terminal.

---

**Ready for more?** [Learn the core concepts →](/docs/concepts)
