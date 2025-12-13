# DEVSPIN

Devspin, a Development Environment Manager.

## What ? 
- Devspin manage your project dependencies, services, ci/cd, environment variables, and more in a **SINGLE** config file (named devspin.yml)
- It implement default features like a linter a formatter and a test generator/runner (more incoming) 
- It is available via a CLI tool (devspin-cli) a GUI (devspin desktop) and also specialized features available via editor extensions 

## Why using devspin ?
- **Simplicity first** : It manages all in one interpreted file (interpretor can be added as an executable in PATH) that can be **autowritten** by devspin itself
- **content agnostic** : It can be used fore every kind of projects, the interpretor does not care about what is inside your project (once formatted)
- **portable** : It can be used on any platform and integrate a docker wrapper
- **containerization** : It use a docker wrapper to manage dependencies and services to appreciate container benefits
- **interpretor** : The interpretor act as a go.mod, package.json, Cargo.toml ... But with your whole project — The interpretor write dockerfile run it config etc...

## How to use it ?
- **CLI** : 
    - install devspin-cli (TODO: add install instructions)
    
    - ```bash
    devspin init # generate a devspin.yml with a sample content
    devspin config # Configure devspin.yml regarding to your project dependencies,  # services, ci/cd, environment variables, and more — you can also integrate yourself or indicate the details in semi_auto_mode(TODO: link to doc)
    devspin run # Run devspin.yml
    ```

- **GUI** : 
    - install devspin-desktop (TODO: add install instructions)
    - ```bash
    devspin-desktop
    ```
    
- **Editor extensions** : 
    - install devspin-editor (TODO: add install instructions)


# LICENSE
MIT

# CONTRIBUTING
if you have any suggestion pls give them to mepastacc@gmail.com