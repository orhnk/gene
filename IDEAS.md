# Ideas

## 10.12.2023

- [Two-Way Interpretation](#two-way-interpretation) of package manager scripts (e.g. `apt-get` and `pacman`) with
  automated [name patching](#name-patching)

## 15.12.2023

- Easy way of publishing packages through GENE. Just like cross compiling in rust
  `gene --publish .` but this feature would require a lot of work.

> Some Package Manager problems
>
> - devel packages not being separate
> - version mismatch
> - non-existent packages
> - different packages with the same name (eg rename vs perl-rename)

## Dictionary

### Name Patching

> The process of translating package names between package managers.

```text
.--------.---------------.
| GPacR  | jasper        |<--.   .--> Err: Package `jasper` not found for your system package manager: `scoop`
|--------|---------------|   |  |
| Nix    | jasper        |---|  |
| APT    | libjasper-dev |---'  |
| Scoop  | NaN           |------'
'--------'---------------'
```

> https://unix.stackexchange.com/questions/107246/is-there-a-map-of-package-names-across-the-different-package-management-systems

### Two Way Interpretation

> The process of translating package manager scripts between package managers.

```text
.------------.---------------.      
| Chocolatey | choco install |---.  .-----------------------------------------.  .-------------.
| Pacman     | pacman -S     |---|--| GENE-TWIN (GENE Two-Way Interpretation) |--| YOUR SYSTEM |---> xbps-install
| APT        | apt install   |---'  '-----------------------------------------'  '-------------'
'------------'---------------'                                                    
```
