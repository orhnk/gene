# GENE Package Manager

GENE (GENEric package manager) is a package manager management system.

### What is the term "Package Manager Manager"?

I like to call it package 2xmanager. GENE targets general popular package managers
(See [Supported Package Managers](#supported-package-managers)) by wrapping the binaries
directly from your system shell. So GENE can do anything you will do by hand.

## Manifest

Package managers are not standardized which can be a problem when you are installing
or even searching the package you want. Maybe you were able to find it for X package manager
but not yours. At least I have been facing this issue some time (especially when installing C libraries)

There are several problems that GENE tries to resolve:

- Package manager's installation system differ by one to another. Standardizing small commands will help users to adapt other package managers easily.
- Package names are not standardized which is a major problem on general system package managers that requires time and effort to get resolved.

## TL;DR

Here is a quick representation on how does GENE manage and standardize package managers

Every machine that runs GENE should include a config file containing the information about the system
and optionally a configuration.

```toml
# $XDG_CONFIG_HOME/gene/gene.toml
# Example GENE configuration
[system]
backends = [
    "apt",
]
```

To install a package using GENE, you have to declare them first (Direct installation is present on [Appendix I](#appendix-i))
As an example I will write the build scripts for my project [Vimacs](https://github.com/utfeight/vimacs).

> [!NOTE]
> the syntax is just the same.
> It's merged with your system configuration before
> GENE starts installing the packages.

```toml
# $VIMACS/gene.toml
[system]
deps = [
    "tuir",
    "mapscii",
    "weechat",
    "neovim-nightly",
    # ...
]
```

inside `$VIMACS/` (where the project's `gene.toml` exists) run:

```shell
gene install
```

and gene will start to install everything present on `gene.toml` as it should.

### Plugin Support

One of the GENE's most powerful feature is It's plugin system.
Because everything is written in rust, installing a plugin requires recompilation of GENE
(Or maybe It'll be in lua to make it easier to maintain the plugin system and usability)

The plugin specific standards are not planned yet (Also this feature will be added later on development)

> [!NOTE]
> All of these are but nothing thoughts.
> They are not implemented yet.

#### What are the benefits of the "plugin system" for a package 2xmanager?

Installing packages are boring. They do the same thing everytime! (That's the good-case btw.)
There are a lot of functionality that you could inject into your package manager using just by using
custom scripts.

#### Plugin Installation

Installing a plugin manager is super simple. Just put it on you `system.plugins` as shown below:

```toml
# $XDG_CONFIG_HOME/gene.toml
[system]
plugins = [
    "binder",
]

[plug.binder.config] # Plugin specific configuration
backend = "git"
verbose = true
prettify = false
ban = [
    "pip",
    "npm",
]
```

Then running any `gene` command from shell will trigger plugin installation process.
You can just type `gene` to just install it.

Here are some example plugin ideas that I've come with.

#### Binder

After installing a dependency from a registry you have the ability to wrap it using language aware binding generators (e.g cbindgen)
To address potential project-specific errors that may occur this plugin takes an interactive approach to resolve issues related to non-standardization
problems with the help of your assistance.

#### Virtualized

This plugin adds functionality to install packages to your virtual machines. (Or maybe servers over SSH)

#### Delete-Config

This plugin adds functionality to remove related configuration when you delete a package.

#### Prettified

This plugin wraps GENE with a TUI and adds theming options.

#### Hooker

Hook your scripts into GENE processes.

### Supported Package Managers

> [!NOTE]
> GENE Project is in planning stage.

### Similar Projects

- whohas - A system utility to search from general package registries.
- pacaptr - pacman-like syntax for all package managers
- bedrock linux's package manager manager - also known as `pmm`


  ```text
  Bedrock linux's package manager manager is one of the closest projects
  to GENE. But has a major drawback: It's also meant to get used as a 
  package manager of an unpopular linux distribution, bedrock linux, 
  that drastically limits It's potential.
  ```

## Building

```shell
cargo build --release --bin gene
```

## Appendix

### Appendix I

This feature is heavily discouraged as It's harder to maintain your system packages.

```shell
gene -f install <package>
gene -f remove <package>
```
