# PPM

PPM - Package Manager Manager

Contains utility for abstracted dependency management (ADM) embracing KISS philosophy.

## The Slang

- SPM - System Package Manager: System-wide dependency manager as extensions to your operating system.
- LPM - Language Package Manager: dependency manager for a programming language (e.g pip, cargo, vcpkg, npm)
- PDM - Project Dependency Manager: Application-specific dependency manager (e.g lazy.nvim, Chrome Extension Store,
  Conda)

## Adding a new Package Manager

1. Add a new rust module in `gene-pmm/src/<pm_type>/<pm_name>/<pm_name>.rs` and implement the `PackageManager` trait.
2. Add your package manager to `gene-config/backends.rs`
