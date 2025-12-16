---
title: Quick Start
description: Get spin running in 5 minutes
slug: /quickstart
sidebar_position: 2
---

#  Quick Start

Get spin running in your environment in less than 5 minutes. By the end of this guide, you'll have a working application that spin manages completely.

## 📋 Prerequisites

- **Node.js 18+** (for the example app)
- **Terminal/Command Prompt**
- **Internet connection** (to download spin)

No Docker, Kubernetes, or cloud accounts required for this quick start!

## 1. Install spin CLI

Choose your installation method:

### Option A: One-line command (Recommended)

```bash
curl -fsSL https://spin.dev/install.sh | sh
```

This downloads and installs the spin CLI to `~/.spin/bin/spin`.

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

## 2. Create Your First Project including spin

Spin can initialize a project for you:

```bash
# Create a new directory
mkdir my-first-spin-app
cd my-first-spin-app

# Initialize with spin
spin init # you can add flags to choose templates 
```

This creates:
- `app.spn` - Your application manifest
- Basic project structure

**IMPORTANT**
  The spin file get shortcuted so make sure you like the name (you can change it with no consequences)

### Manual Creation (Alternative)

If you prefer to create files manually:

```bash
mkdir my-first-spin-app
cd my-first-spin-app
```

Create `app.spn`:

```spn
#! spn 1.0
cfg{ 
  name: "my-first-spin-app",
  version: 1.0.0,
}

@service>web | ; pipe is necessary
@runtime>node |
@interpret>dist
{
  port: 3000
  run: "npm start"
}
```

So here:
1. versioning and configs are minimal <code>#! spn 1.0</code> stand for the interpretor version
<code>\{ name: "my-first-spin-app" \}</code> is a minimal config (more is allowed)

2. @ chain is targets: Here, it first say that this a service (not an app for scalability) .Then, it tells what is the current runtime, so nodejs. Finnaly, it tells the intepretor that he has to interpret it and put the output in dist/ that gets run in the main Docker file


Create a simple `package.json`:

```json
{
  "name": "my-first-spin-app",
  "version": "1.0.0",
  "scripts": {
    "start": "node server.js"
  },
  "dependencies": {
    "express": "^4.18.2"
  }
}
```
Here, you create a simple package.json that defines version of the app, name, the scripts and the dependencies


Create `server.js`:

```javascript
const express = require('express');
const app = express();
const port = process.env.PORT || 3000;

app.get('/', (req, res) => {
  res.send(`
    <!DOCTYPE html>
    <html>
      <head><title>Hello spin!</title></head>
      <body style="font-family: Arial, sans-serif; text-align: center; padding: 50px;">
        <h1> Hello from spin!</h1>
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

Now for the magic! Run your app with spin:

```bash
spin run
```

**What happens next:**

1. **spin analyzes** your `app.spn` file
2. **Detects** you have a Node.js app
3. **Generates** a `Dockerfile` (if Docker is available)
4. **Creates** `docker-compose.yml` for orchestration
5. **Starts** your app with hot-reload

You should see output like:

```
spin detected Node.js project
Generated Dockerfile
Starting application...
  Web server: http://localhost:3000
```
You can customize the output by using flags and devSpin (coming soon...)

## 4. Verify It's Working

Open your browser to **http://localhost:3000**

You should see a page that says "Hello from SPIN!" with the current time.

## 5. Explore What spin Created

spin generates platform-specific configs for you. Check what was created:

```bash
cd dist/
ls -la
```

You might see:
- `Dockerfile` - Container definition
- `.spin/` - spin working directory

## 🎉 Congratulations!

You've successfully:
- ✅ Installed spin CLI
- ✅ Created an application manifest
- ✅ Run your first spin-managed app
- ✅ Seen hot-reload in action

## 🚀 Next Steps

Now that you have spin working, explore:

- **[Core Concepts](/docs/concepts)** - Learn about targets, modes, and builtins
- **[First Real Project](/docs/first-project)** - Migrate an existing application
- **[FAQ](/docs/faq)** - Common questions and troubleshooting

## 🆘 Having Issues?

- Check `spin --help` for command options
- Run `spin validate` to check your spin file
- Join our [Discord Community](https://discord.gg/spin) for help

**Need to stop the app?** Press `Ctrl+C` in your terminal.

---

**Ready for more?** [Learn the core concepts →](/docs/concepts)
