# GCM - GENE Config Manager

Separation of GENE Package Manager Configuration.

## TL;DR

GENE is a declarative system-wide package 2xmanager (See main [README.md](../README.md) for more information)

It uses `TOML` markup language for configuration file format.

This module is responsible for parsing, validating and merging (+linking) configuration files.

GENE requires at least one configuration file to be present in the system to work properly.

## Configuration

> [!NOTE]
> Minimum configuration requires at least one `system.backends` to be present.
> As below a backend stands for a package manager that [you have on your $PATH](#is-program-x-in-my-path).

```toml
# $XDG_CONFIG_HOME/gene/config.toml
[system]
backends = [
	"apt",
]
```

After you have configured your `config.toml`, you can run `gene` to install packages.

There are two ways to run `gene`:

### Declarative

> This is the recommended way to use `gene`.

> [!NOTE]
> Put `gene.toml` in your project root directory.
> Also there is no difference between local and global configuration for GENE. (so `$XDG_CONFIG_HOME/gene/config.toml`
> is overridden by `$MY_PROJECT/gene.toml`)

```toml
# $MY_PROJECT/gene.toml
```

#### Merge Semantics

GENE configurations are merged in the following order:

1. `$XDG_CONFIG_HOME/gene/config.toml`
2. `$MY_PROJECT/gene.toml`

This allows you to override system-wide configuration with your project configuration.
Which is useful for projects that require specific package managers.

### Dumb(ish)

> [!WARNING]
> This method is not recommended for production use.
> Because it is hard to track what packages are installed on your system.

> [!NOTE]
> Replace `<package>` with the package you want to install.

```bash
gene install -i <package>
```

## Appendix

### `$PATH`?

`$PATH` is an environment variable that contains a list for your shell REPL prelude.
Just like how you have pre-defined keywords like `import` in some programming languages.

For more information, please refer to [Wikipedia](https://en.wikipedia.org/wiki/PATH_(variable)).

#### How can I check my `$PATH`?

##### What are my directories in `$PATH`?

```bash
echo $PATH # list all directories in your $PATH
```

##### Is program `X` in my `$PATH`?

just type `X` in your shell REPL. If it's in your `$PATH`, it will be executed.

```bash
X # if X is in your $PATH, it will be executed
```



